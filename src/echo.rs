use tokio::io::{AsyncReadExt, AsyncWriteExt};
use tokio::net::{TcpListener, TcpStream};

#[tokio::main]
async fn main() {
    let listener = TcpListener::bind("127.0.0.1:6000").await.unwrap();
    println!("TCP echo server listening on 127.0.0.1:6000");
    loop {
        // accept incoming connection
        match listener.accept().await {
            Ok((stream, addr)) => {
                println!("new client: {}", addr);
                // spawn task for each client, concurrent
                tokio::spawn(async move {
                    handle_client(stream).await;
                    println!("client {} disconnected", addr);
                });
            }
            Err(err) => {
                println!("err:{:?}", err);
            }
        }
    }
}

async fn handle_client(mut stream: TcpStream) {
    let mut buf = [0u8; 1024];
    loop {
        // read bytes from client
        let n = match stream.read(&mut buf).await {
            Ok(0) => {
                // read 0 bytes = client close connection
                return;
            }
            Ok(n) => n,
            Err(e) => {
                eprintln!("read error: {}", e);
                return;
            }
        };

        let data = String::from_utf8_lossy(&buf[..n]).to_lowercase();
        println!("length:{}=={:?}", data.len(), data.as_bytes());
        if data.trim() == "exit" {
            println!("client requested exit");
            return;
        }

        // echo back: write received bytes back to stream
        if stream.write_all(&buf[..n]).await.is_err() {
            eprintln!("write error");
            return;
        }
        if let Err(err) = stream.flush().await {
            eprintln!("failed to flush: {}", err);
            return;
        }
    }
}
