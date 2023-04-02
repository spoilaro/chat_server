use futures::Stream;
use std::error::Error;
use std::sync::Arc;
use std::{collections::HashMap, net::SocketAddr};
use tokio::io::{AsyncBufReadExt, AsyncRead, AsyncWriteExt, BufReader};
use tokio::net::{TcpListener, TcpStream};
use tokio::sync::{broadcast, Mutex};

struct Client {
    address: String,
    nickname: String,
}
struct ChatState {
    clients: Vec<Client>,
}

impl ChatState {
    fn new() -> ChatState {
        ChatState { clients: vec![] }
    }
}

#[tokio::main]
async fn main() {
    let address = "localhost:8001";

    let (tx, rx) = broadcast::channel(10);
    let listener = TcpListener::bind(address).await.unwrap();

    loop {
        let (mut socket, addr) = listener.accept().await.unwrap();
        let tx = tx.clone();
        let mut rx = tx.subscribe();

        // let state = Arc::clone(&state);

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

                        writer.write_all(msg.as_bytes()).await.unwrap();
                        }
                    }


                }
            }
        });
    }
}
