use std::error::Error;
use tokio::io::{AsyncReadExt, AsyncWriteExt};
use tokio::net::{TcpListener, TcpStream};

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    // Listen on port 54000
    let listener = TcpListener::bind("0.0.0.0:54000").await?;

    println!("Listening on port 54000");

    loop {
        let (socket, _) = listener.accept().await?;
        process_socket(socket).await?;
    }
}

async fn process_socket(mut socket: TcpStream) -> Result<(), Box<dyn Error>> {
    // Accept TCP connections.

    // Whenever you receive data from a client, send it back unmodified.

    // Make sure you don't mangle binary data, and that you can handle at
    // least 5 simultaneous clients.

    // Once the client has finished sending data to you it shuts down its
    // sending side. Once you've reached end-of-file on your receiving side,
    // and sent back all the data you've received, close the socket so that
    // the client knows you've finished.

    let mut buffer = Vec::new();

    socket.read_to_end(&mut buffer).await?;

    socket.write_all(&buffer).await?;

    Ok(())
}
