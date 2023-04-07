use std::io::Write;
use std::result;
use tokio::io::{AsyncBufReadExt, AsyncWriteExt, BufReader};
use tokio::net::TcpStream;

/// Reads stdin for the name & removes the newline
fn get_name() -> String {
    let mut line = String::new();
    let stdin = std::io::stdin();

    print!("Input your name: ");
    std::io::stdout().flush().unwrap();
    let bytes_read = stdin.read_line(&mut line).unwrap();
    line = line.replace("\n", "");
    println!("Bytes read: {}", bytes_read);

    line
}

/// Reads stdin for the server address
fn get_address() -> String {
    let mut line = String::new();
    let stdin = std::io::stdin();

    print!("Address: ");
    std::io::stdout().flush().unwrap();
    let bytes_read = stdin.read_line(&mut line).unwrap();
    line = line.replace("\n", "");
    println!("Bytes read: {}", bytes_read);

    line
}

#[tokio::main]
async fn main() {
    let name = get_name();
    let address = get_address();

    let mut stream = TcpStream::connect("localhost:8001").await.unwrap();

    // Reader & Writer for the stream
    let (reader, mut writer) = stream.split();
    let mut reader = BufReader::new(reader);
    let mut line = String::new();

    // User reader
    let mut user_reader = BufReader::new(tokio::io::stdin());
    let mut user_line = String::new();

    loop {
        tokio::select! {

            // Receives the messages
            result = reader.read_line(&mut line) => {
                if result.unwrap() == 0 {
                    break;
                }
                println!("{}", line);
                line.clear();
            }

            // Sends the message
            _result = user_reader.read_line(&mut user_line) => {


                if user_line.contains("exit()") {
                    writer.shutdown().await.unwrap();
                    break;
                }

                writer.write_all(user_line.clone().as_bytes()).await.unwrap();

            }

        }
    }

    println!("Closing the chat, good bye!");
}
