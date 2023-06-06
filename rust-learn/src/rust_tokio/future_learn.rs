use futures::task::{Waker, Context, Poll, ArcWake, waker_ref};
use std::sync::{Arc, Mutex};
use futures::{Future, FutureExt};
use tokio::macros::support::Pin;
use tokio::time::Duration;
use std::thread;
use futures::future::{BoxFuture, ok};
use std::sync::mpsc::{SyncSender, sync_channel,Receiver};
use std::time::Duration;


pub struct TimeFuture{
    shared_state: Arc<Mutex<SharedState>>,
}

struct SharedState{
    completed: bool,
    waker: Option<Waker>,
}

impl Future for TimeFuture{
    type Output = ();

    fn poll(self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Self::Output> {
        let mut shared_state = self.shared_state.lock().unwrap();
        if shared_state.completed {
            Poll::Ready(())
        }else{
            shared_state.waker = Some(cx.waker().clone());
            Poll::Pending
        }
    }
}

impl TimeFuture {
    pub fn new(duration:Duration) -> Self{
        let shared_state = Arc::new(Mutex::new(SharedState{
            completed:false,
            waker:None,
        }));
        let thread_shared_state = shared_state.clone();
        thread::spawn(move||{
            thread::sleep(duration);
            let mut guard = thread_shared_state.lock().unwrap();
            guard.completed = true;
            if let Some(waker) = guard.waker.take(){
                waker.wake();
            }
        });
        TimeFuture{
            shared_state,
        }
    }
}

struct Excutor{
    ready_queue: Receiver<Arc<Task>>,
}

#[derive(Clone)]
struct Spawner{
    task_sender:SyncSender<Arc<Task>>,
}

struct Task{
    future:Mutex<Option<BoxFuture<'static,()>>>,
    task_sender:SyncSender<Arc<Task>>,
}

fn new_excutor_and_spawner()->(Excutor,Spawner){
    const MAX_QUEUED_TASKS:usize = 10_000;
    let (task_sender,ready_queue) = sync_channel(MAX_QUEUED_TASKS);
    (Excutor{ready_queue},Spawner{task_sender})
}

impl Spawner{
    fn spawner(&self,future:impl Future<Output=()>+'static+Send){
        let future = future.boxed();
        let task = Arc::new(Task{
            future:Mutex::new(Some(future)),
            task_sender:self.task_sender.clone(),
        });
        self.task_sender.send(task).expect("too many tasks queued");
    }
}

impl ArcWake for Task {
    fn wake_by_ref(arc_self: &Arc<Self>) {
        let cloned = arc_self.clone();
        arc_self.task_sender.send(cloned).expect("too many tasks queued");
    }
}

impl Excutor {
    fn run(&self){
        while let Ok(task) = self.ready_queue.recv(){
            let mut future_slot = task.future.lock().unwrap();
            if let Some(mut future) = future_slot.take() {
                let waker = waker_ref(&task);
                let context = &mut Context::from_waker(&*waker);
                if future.as_mut().poll(context).is_pending() {
                    *future_slot = Some(future);
                }
            }
        }
    }
}

#[cfg(test)]
mod test{
    use super::*;

    #[test]
    fn test01(){
        let (excutor,spawner) = new_excutor_and_spawner();
        spawner.spawner(async {
            println!("{}","start!");
            TimeFuture::new(Duration::new(2,0)).await;
            println!("{}","end!");
        });
        drop(spawner);
        excutor.run();
    }
}