
#[cfg(test)]
mod test{
    use std::io::{Read, read_to_string, Write};
    use std::net::{Shutdown, TcpListener, TcpStream};
    use std::thread;
    use std::thread::Thread;
    use tokio::runtime;
    use std::vec;

    #[test]
    fn test(){
        let runtime1 = runtime::Builder::new_multi_thread().build().unwrap();
        for number in 1..100  {
            let future =  async move {
                println!("{}",number);
            };
            runtime1.spawn(future);
        }
        println!("{}","=================ok=================");
        runtime1.spawn(async  {
            println!("{}","ok,ok");
        });
    }

    #[test]
    fn server1(){
        let listener = TcpListener::bind("127.0.0.1:8080").unwrap();
        loop {
            match listener.accept() {
                Ok((mut _socket, addr)) => {
                    println!("new client: {addr:?}");
                    _socket.write(b"server1 hello");
                    let mut buffer = [0; 10];
                    // read up to 10 bytes
                    let n = _socket.read(&mut buffer[..]).unwrap();
                    println!("server1 The bytes: {:?}", &buffer[0..n]);
                },
                Err(e) => println!("couldn't get client: {e:?}"),
            }
        }

    }
    #[test]
    fn server2(){
        let listener = TcpListener::bind("127.0.0.1:8081").unwrap();
        loop {
            match listener.accept() {
                Ok((mut _socket, addr)) => {
                    println!("new client: {addr:?}");
                    _socket.write(b"server2 hello");
                    let mut buffer = [0; 10];
                    // read up to 10 bytes
                    let n = _socket.read(&mut buffer[..]).unwrap();
                    println!("server2 The bytes: {:?}", &buffer[0..n]);
                },
                Err(e) => println!("couldn't get client: {e:?}"),
            }
        }

    }

    #[test]
    fn client1(){
        let mut stream = TcpStream::connect("127.0.0.1:8080").expect("connect failed");

        let mut buffer = String::new();
        // read up to 10 bytes
        stream.write(b"hello");
        stream.flush();
        stream.shutdown(Shutdown::Write);
        let n = stream.read_to_string(&mut buffer);
        println!("The bytes: {:?}", buffer);
    }

    fn client2(){
        let mut stream = TcpStream::connect("127.0.0.1:8081").unwrap();
        let mut buffer = Vec::new();
        // read up to 10 bytes
        let n = stream.read(&mut buffer);
        let string = String::from_utf8(buffer);
        println!("The bytes: {:?}", string);
        stream.write(b"hello");
    }

    #[test]
    fn client_test(){
        let handle1 = thread::spawn(|| {client1()});
        // let handle2 = thread::spawn(|| {client2()});
        let mut handles = Vec::new();
        handles.push(handle1);
        // handles.push(handle2);
        for handle in handles {
            handle.join();
        }
    }

    async fn hello(){
        println!("{}","hello");
    }

    #[test]
    fn test02(){
        futures::executor::block_on(hello());
    }
}