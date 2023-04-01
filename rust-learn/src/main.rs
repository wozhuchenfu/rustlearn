extern crate futures;

use std::net::{TcpListener, TcpStream};
use std::io::{Read, Write};
use std::fs;
use tokio::io::{AsyncReadExt, AsyncWriteExt};
use tokio::net::TcpListener as tokTcpListener;

mod rust_async;
fn handle_connection(mut stream: TcpStream){
    let mut buffer = [0;1024];
    stream.read(&mut buffer).unwrap();
    let get = b"GET / HTTP/1.1\r\n";
    println!("{}",String::from_utf8(buffer.to_vec()).unwrap());
    let (status_line,filename) = if buffer.starts_with(get){
        ("HTTP/1.1 200 OK\r\n\r\n","rust-learn/src/hello.html")
    }else {
        ("HTTP/1.1 404 NOT FOUND\r\n\r\n","rust-learn/src/404.html")
    };
    let contents = fs::read_to_string(filename).unwrap();
    let response = format!("{status_line}{contents}");
    stream.write_all(response.as_bytes()).unwrap();
    stream.flush().unwrap();
}


#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {

    let listener = tokTcpListener::bind("127.0.0.1:8080").await?;
    loop {
        let (mut socket, _) = listener.accept().await?;

        tokio::spawn(async move {
            let mut buf = [0; 1024];

            // In a loop, read data from the socket and write the data back.
            loop {
                let n = match socket.read(&mut buf).await {
                    // socket closed
                    Ok(n) if n == 0 => return,
                    Ok(n) => n,
                    Err(e) => {
                        println!("failed to read from socket; err = {:?}", e);
                        return;
                    }
                };
                println!("buf {}",String::from_utf8(buf.to_vec()).unwrap());
                // Write the data back
                if let Err(e) = socket.write_all(b"hello").await {
                    println!("failed to write to socket; err = {:?}", e);
                    return;
                }
            }
        });
    }
}



fn main1() {
    let listener = TcpListener::bind("127.0.0.1:8080").unwrap();
    for stream in listener.incoming(){
        let stream = stream.unwrap();
        handle_connection(stream);
    }
    use std::collections::HashMap;

    let mut scores = HashMap::new();

    scores.insert(String::from("Blue"), 10);
    scores.insert(String::from("Yellow"), 50);
    //Entry 的 or_insert 方法在键对应的值存在时就返回这个值的可变引用，如果不存在则将参数作为新值插入并返回新值的可变引用
    scores.entry("Blue".to_string()).or_insert(90);
    let a = scores.get("Blue").unwrap();
    println!("{}",a);
    let team_name = String::from("Blue");
    let score = scores.get(&team_name).copied().unwrap_or(0);
    println!("{}",score);
}

