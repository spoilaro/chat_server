use tokio::io::{AsyncBufReadExt, AsyncWriteExt, BufReader};
use tokio::net::TcpListener;
use tokio::sync::broadcast;

#[tokio::main]
async fn main() {
    let address = "localhost:8001";

    let (tx, _rx) = broadcast::channel(10);

    // Binds the address set above
    let listener = TcpListener::bind(address).await.unwrap();

    println!("\nStarting the chat server, address: {}", address);

    loop {
        // Gets the socket and the address of the connected client
        let (mut socket, addr) = listener.accept().await.unwrap();

        // Transmitter needs to be cloned because of the ownership
        let tx = tx.clone();

        // Receiver is not cloned but got through subscribing to the transmitter
        let mut rx = tx.subscribe();

        // New thread for each of the clients
        tokio::spawn(async move {
            let (reader, mut writer) = socket.split();

            let mut reader = BufReader::new(reader);
            let mut line = String::new();

            loop {
                tokio::select! {
                    result = reader.read_line(&mut line) => {
                        if result.unwrap() == 0 {
                            break;
                    }

                        tx.send((line.clone(), addr)).unwrap();
                        line.clear();
                    }

                    result = rx.recv() => {
                        let (msg, other_addr) = result.unwrap();

                        if addr != other_addr {

                            let edited_msg = format!("{}: {}", other_addr, msg);
                            writer.write_all(edited_msg.as_bytes()).await.unwrap();
                        }
                    }


                }
            }
        });
    }
}
