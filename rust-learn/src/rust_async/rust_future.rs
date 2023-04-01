use std::future::Future;
use std::sync::{Arc, Mutex};
use std::rc::Weak;
use std::task::{Waker, Context, Poll};
use std::pin::Pin;
use std::thread;
use std::time::Duration;

pub struct TimerFuture{
    shared_state:Arc<Mutex<SharedState>>
}

struct SharedState{
    completed:bool,
    waker:Option<Waker>
}

impl Future for TimerFuture{
    type Output=();
    fn poll(self:Pin<&mut Self>,cx:&mut Context<'_>)->Poll<Self::Output>{
        let mut shared_state = self.shared_state.lock().unwrap();
        if shared_state.completed {
            Poll::Ready(())
        }else {
            shared_state.waker = Some(cx.waker().clone());
            Poll::Pending
        }
    }
}

impl TimerFuture{
    pub fn new(duration:Duration)->Self{
        let shared_state = Arc::new(Mutex::new(SharedState{
            completed:false,
            waker:None
        }));
        //生成新线程
        let thread_shared_state = shared_state.clone();
        thread::spawn(move||{
            thread::sleep(duration);
            let mut shared_state = thread_shared_state.lock().unwrap();
            //发出信号：计时器已停止并唤醒 future被poll的最后一个任务
            shared_state.completed = true;
            if let Some(waker) = shared_state.waker.take(){
                waker.wake();
            };
        });
        TimerFuture{shared_state}
    }
}

#[cfg(test)]
mod test{

    use super::*;
    use futures::{
        future::{BoxFuture, FutureExt},
        task::{waker_ref, ArcWake},
    };
    use std::{
        future::Future,
        sync::mpsc::{sync_channel, Receiver, SyncSender},
        sync::{Arc, Mutex},
        task::Context,
        time::Duration,
    };

    /// Task executor that receives tasks off of a channel and runs them.
    struct Executor {
        ready_queue: Receiver<Arc<Task>>,
    }

    /// `Spawner` spawns new futures onto the task channel.
    #[derive(Clone)]
    struct Spawner {
        task_sender: SyncSender<Arc<Task>>,
    }

    /// A future that can reschedule itself to be polled by an `Executor`.
    struct Task {
        /// In-progress future that should be pushed to completion.
        ///
        /// The `Mutex` is not necessary for correctness, since we only have
        /// one thread executing tasks at once. However, Rust isn't smart
        /// enough to know that `future` is only mutated from one thread,
        /// so we need to use the `Mutex` to prove thread-safety. A production
        /// executor would not need this, and could use `UnsafeCell` instead.
        future: Mutex<Option<BoxFuture<'static, ()>>>,

        /// Handle to place the task itself back onto the task queue.
        task_sender: SyncSender<Arc<Task>>,
    }

    fn new_executor_and_spawner() -> (Executor, Spawner) {
        // Maximum number of tasks to allow queueing in the channel at once.
        // This is just to make `sync_channel` happy, and wouldn't be present in
        // a real executor.
        const MAX_QUEUED_TASKS: usize = 10_000;
        let (task_sender, ready_queue) = sync_channel(MAX_QUEUED_TASKS);
        (Executor { ready_queue }, Spawner { task_sender })
    }

    impl Spawner {
        fn spawn(&self, future: impl Future<Output = ()> + 'static + Send) {
            let future = future.boxed();
            let task = Arc::new(Task {
                future: Mutex::new(Some(future)),
                task_sender: self.task_sender.clone(),
            });
            self.task_sender.send(task).expect("too many tasks queued");
        }
    }
    impl ArcWake for Task {
        fn wake_by_ref(arc_self: &Arc<Self>) {
            // Implement `wake` by sending this task back onto the task channel
            // so that it will be polled again by the executor.
            let cloned = arc_self.clone();
            arc_self
                .task_sender
                .send(cloned)
                .expect("too many tasks queued");
        }
    }

    impl Executor {
        fn run(&self) {
            while let Ok(task) = self.ready_queue.recv() {
                // Take the future, and if it has not yet completed (is still Some),
                // poll it in an attempt to complete it.
                let mut future_slot = task.future.lock().unwrap();
                if let Some(mut future) = future_slot.take() {
                    // Create a `LocalWaker` from the task itself
                    let waker = waker_ref(&task);
                    let context = &mut Context::from_waker(&waker);
                    // `BoxFuture<T>` is a type alias for
                    // `Pin<Box<dyn Future<Output = T> + Send + 'static>>`.
                    // We can get a `Pin<&mut dyn Future + Send + 'static>`
                    // from it by calling the `Pin::as_mut` method.
                    if future.as_mut().poll(context).is_pending() {
                        // We're not done processing the future, so put it
                        // back in its task to be run again in the future.
                        *future_slot = Some(future);
                    }
                }
            }
        }
    }


    #[test]
    fn test(){
        let (executor, spawner) = new_executor_and_spawner();

        // Spawn a task to print before and after waiting on a timer.
        spawner.spawn(async {
            println!("howdy!");
            // Wait for our timer future to complete after two seconds.
            TimerFuture::new(Duration::new(2, 0)).await;
            println!("done!");
        });

        // Drop the spawner so that our executor knows it is finished and won't
        // receive more incoming tasks to run.
        drop(spawner);

        // Run the executor until the task queue is empty.
        // This will print "howdy!", pause, and then print "done!".
        executor.run();
    }

}