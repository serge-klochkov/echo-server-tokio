use std::time::Duration;
use tokio::io::{AsyncReadExt, AsyncWriteExt};
use tokio::net::{TcpListener, TcpStream};

use echo_server_tokio::ECHO_SERVER_ADDRESS;

#[tokio::main]
async fn main() {
    let listener = TcpListener::bind(ECHO_SERVER_ADDRESS).await.unwrap();
    loop {
        let (stream, _) = listener.accept().await.unwrap();
        tokio::spawn(async move {
            handle_connection(stream).await;
        });
    }
}

async fn handle_connection(mut stream: TcpStream) {
    let mut buffer = [0; 1024];
    let len = stream.read(&mut buffer).await.unwrap();
    let message = String::from_utf8_lossy(&buffer[..len]);
    println!("Received message {} with length {}", &message, len);
    std::thread::sleep(Duration::from_millis(1000));
    println!("Delayed 1000ms for message {}", &message);
    stream.write(&message.as_bytes()).await.unwrap();
    stream.flush().await.unwrap();
}
