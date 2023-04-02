use tokio::io::{AsyncReadExt, AsyncWriteExt};

#[tokio::main]
async fn main() {
    let listener = tokio::net::TcpListener::bind("localhost:8001")
        .await
        .unwrap();

    let (mut socket, _addr) = listener.accept().await.unwrap();

    loop {
        let mut buffer = [0u8; 1024];
        let bytes_read = socket.read(&mut buffer).await.unwrap();

        socket.write_all(&buffer[..bytes_read]).await.unwrap();
    }
}
