use std::sync::Arc;

use tokio::io::{AsyncBufReadExt, AsyncWriteExt, BufReader};
use tokio::net::TcpListener;
use tokio::sync::{broadcast, Mutex};

struct ChatClient {
    addr: String,
}

struct ChatState {
    clients: Vec<ChatClient>,
}

impl ChatClient {
    fn new(addr: String) -> ChatClient {
        ChatClient { addr }
    }
}

impl ChatState {
    fn new() -> ChatState {
        ChatState { clients: vec![] }
    }

    fn new_client(&mut self, c: ChatClient) {
        println!("New client: {}", c.addr);
        self.clients.push(c);
    }
}

#[tokio::main]
async fn main() {
    let address = "localhost:8001";
    let (tx, _rx) = broadcast::channel(10);

    // Binds the address set above
    let listener = TcpListener::bind(address).await.unwrap();

    println!("\nStarting the chat server, address: {}", address);

    let state = Arc::new(Mutex::new(ChatState::new()));

    loop {
        let state = Arc::clone(&state);

        // Gets the socket and the address of the connected client
        let (mut socket, addr) = listener.accept().await.unwrap();

        // let mut state = state.lock().await;

        // state.new_client(ChatClient::new(addr.to_string().to_string()));
        //
        // for client in state.clients.iter() {
        //     println!("{} -> Connected", client.addr);
        // }

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
            tx.send((format!("New client connected: {}\n", addr), addr))
                .unwrap();
            line.clear();

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

                            writer.write_all(msg.as_bytes()).await.unwrap();
                        }
                    }


                }
            }
        });
    }
}
