use rand::distributions::Alphanumeric;
use rand::Rng;
use tokio::io::AsyncWriteExt;
use tokio::net::TcpStream;

use echo_server_tokio::ECHO_SERVER_ADDRESS;

const MESSAGE: &str = "Hello, world!";

#[tokio::main]
async fn main() {
    match TcpStream::connect(ECHO_SERVER_ADDRESS).await {
        Ok(mut stream) => {
            let local_addr = stream.local_addr().expect(&format!(
                "Failed to get local addr for {}",
                ECHO_SERVER_ADDRESS
            ));
            println!(
                "Connected to echo server {}:{}",
                local_addr.ip(),
                local_addr.port()
            );

            // send
            let id = generate_id();
            stream
                .write(format!("{} (id {})", MESSAGE, id).as_bytes())
                .await
                .expect("Failed to write into the stream");
            stream.flush().await.expect("Failed to flush the stream");
        }
        Err(_) => {
            eprintln!("Failed to connect to echo server {}", ECHO_SERVER_ADDRESS)
        }
    }
}

fn generate_id() -> String {
    rand::thread_rng()
        .sample_iter(&Alphanumeric)
        .take(8)
        .map(char::from)
        .collect()
}
