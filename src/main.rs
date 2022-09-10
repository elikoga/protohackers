use std::error::Error;
use tokio::{
    io::{AsyncReadExt, AsyncWriteExt},
    net::{TcpListener, TcpStream},
};

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

#[derive(Debug)]
enum ReceiveMessage {
    Insert { timestamp: i32, price: i32 },
    Query { mintime: i32, maxtime: i32 },
}

async fn process_socket(mut socket: TcpStream) -> Result<(), Box<dyn Error>> {
    // split
    let (mut reader, mut writer) = socket.split();

    let mut message_buffer = Vec::new();

    loop {
        // read 9 bytes
        let mut buf = [0; 9];
        reader.read_exact(&mut buf).await?;
        // parse
        let message = parse_message(&buf)?;
        // println!("Parsed message: {:?}", message);
        match message {
            ReceiveMessage::Insert { timestamp, price } => {
                message_buffer.push((timestamp, price));
            }
            ReceiveMessage::Query { mintime, maxtime } => {
                let applicable_prices: Vec<i64> = message_buffer
                    .iter()
                    .filter(|(timestamp, _)| *timestamp >= mintime && *timestamp <= maxtime)
                    .map(|(_, price)| (*price).into())
                    .collect::<Vec<_>>();
                // if applicable_messages is empty, then send 0_i32
                if applicable_prices.is_empty() {
                    writer.write_all(&0_i32.to_be_bytes()).await?;
                    continue;
                }
                // calculate mean
                let mean: i32 = (applicable_prices.iter().sum::<i64>()
                    / applicable_prices.len() as i64)
                    .try_into()
                    .unwrap();
                // write mean
                writer.write_all(&mean.to_be_bytes()).await?;
            }
        }
    }
}

fn parse_message(data: &[u8; 9]) -> Result<ReceiveMessage, Box<dyn Error>> {
    let message_type = data[0];
    let first_number = i32::from_be_bytes([data[1], data[2], data[3], data[4]]);
    let second_number = i32::from_be_bytes([data[5], data[6], data[7], data[8]]);
    match message_type {
        b'I' => Ok(ReceiveMessage::Insert {
            timestamp: first_number,
            price: second_number,
        }),
        b'Q' => Ok(ReceiveMessage::Query {
            mintime: first_number,
            maxtime: second_number,
        }),
        _ => Err("Invalid message type".into()),
    }
}
