use serde::{Deserialize, Serialize};
use std::error::Error;
use tokio::io::{AsyncBufReadExt, AsyncReadExt, AsyncWriteExt, BufReader, BufStream};
use tokio::net::{TcpListener, TcpStream};

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    // Listen on port 54000
    let listener = TcpListener::bind("0.0.0.0:54000").await?;

    println!("Listening on port 54000");

    loop {
        let (socket, _) = listener.accept().await?;
        tokio::spawn(async move {
            process_socket(socket).await.unwrap();
        });
    }
}

#[derive(Serialize, Deserialize, Debug)]
struct ReceiveMessage {
    method: Method,
    number: serde_json::Number,
}

#[derive(Serialize, Deserialize, Debug)]
enum Method {
    #[serde(rename = "isPrime")]
    IsPrime,
}

#[derive(Serialize, Deserialize)]
struct SendMessage {
    method: Method,
    prime: bool,
}

async fn process_socket(mut socket: TcpStream) -> Result<(), Box<dyn Error>> {
    let (mut reader, mut writer) = socket.split();
    let buf_reader = BufReader::new(&mut reader);

    let mut lines = buf_reader.lines();

    while let Some(line) = lines.next_line().await? {
        // parse as ReceiveMessage
        let message = serde_json::from_str::<ReceiveMessage>(&line);
        println!("Received: {:?}", message);
        // if invalid, send back error
        if message.is_err() {
            eprintln!("Invalid message: {}", line);
            eprintln!("Error: {}", message.err().unwrap());
            let error = serde_json::json!({
                "error": "Invalid message"
            });
            writer.write_all(error.to_string().as_bytes()).await?;
            // close connection
            return Ok(());
        }
        let message = message.unwrap();
        // check if number is prime
        let is_prime = is_prime(message.number);
        // send back result
        let result = SendMessage {
            method: Method::IsPrime,
            prime: is_prime,
        };
        writer
            .write_all(serde_json::to_string(&result)?.as_bytes())
            .await?;
        // send newline
        writer.write_all(b"\n").await?;
    }

    Ok(())
}

fn is_prime(number: serde_json::Number) -> bool {
    // if non-integral, not prime
    if !number.is_u64() {
        return false;
    }
    let n = number.as_u64().unwrap();
    if n <= 1 {
        return false;
    }
    if n <= 3 {
        return true;
    }
    if n % 2 == 0 || n % 3 == 0 {
        return false;
    }
    let mut i = 5;
    while i * i <= n {
        if n % i == 0 || n % (i + 2) == 0 {
            return false;
        }
        i += 6;
    }
    true
}
