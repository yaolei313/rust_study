use std::io;
use std::io::prelude::*;
use std::io::BufReader;
use tokio::io::AsyncReadExt;
use tokio::net::{TcpListener, TcpStream};

#[tokio::main]
async fn main() {
    let listener = TcpListener::bind("127.0.0.1:8081").await.expect("bind :8081 failed");
    loop {
        let (socket, _) = listener.accept().await.expect("accept fail");
        tokio::spawn(async move {
            handle_connect(socket).await;
        });
    }
}

async fn handle_connect(mut socket: TcpStream) -> io::Result<()> {
    let mut buf = [0; 8 * 1024];
    loop {
        let n = socket.read(&mut buf).await?;
        if n == 0 {
            break;
        }
    }
    Ok(())
}

async fn echo_handle(mut socket: TcpStream) -> io::Result<()> {
    let (r, w) = socket.split();

    Ok(())
}
