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

            // Send the new client connected message
            let msg = format!("New client connected: {}\n", addr);
            tx.send((msg.clone(), addr)).unwrap();
            line.clear();
            println!("{}", msg);

            loop {
                tokio::select! {
                    result = reader.read_line(&mut line) => {

                        // If receives the shutdown signal, break the loop and finish the
                        // task/thread
                        if result.unwrap() == 0 {
                            println!("Client ({}) left", addr);
                            break;
                    }
                        tx.send((line.clone(), addr)).unwrap();
                        println!("{}", line);
                        line.clear();
                    }

                    result = rx.recv() => {
                        let (msg, other_addr) = result.unwrap();
                        if addr != other_addr {
                            writer.write_all(msg.as_bytes()).await.unwrap();
                        }
                    }


                }
            }
        });
    }
}
