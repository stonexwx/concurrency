use std::result::Result::Ok;
use anyhow::Result;
use tokio::{io, net::TcpListener};
use tokio::io::AsyncWriteExt;
use tracing::{info, warn};

const MAX_BUFFER_SIZE: usize = 4096;

#[tokio::main]
async fn main() -> Result<()> {
    tracing_subscriber::fmt::init();
    // build a listener

    let addr = "0.0.0.0:6379";

    let listener = TcpListener::bind(addr).await?;
    info!("Listening on: {}", addr);

    loop {
        let (socket, raddr) = listener.accept().await?;
        info!("Accepted connection from: {}", raddr);
        tokio::spawn(async move {
            if let Err(e) = process_redis_connection(socket).await{
                warn!("Error: {}", e);
            }
        });
    }
}

async fn process_redis_connection(mut stream: tokio::net::TcpStream) -> Result<()> {
    loop {
        stream.readable().await?;
        let mut buffer: Vec<u8> = Vec::with_capacity(MAX_BUFFER_SIZE);

        match stream.try_read_buf(&mut buffer) {
            Ok(0) => break,
            Ok(n) => {
                info!("read {} bytes", n);
                let line = String::from_utf8_lossy(&buffer);
                info!("Received: {}", line);
                stream.write_all(b"+OK\r\n").await?;
            }
            Err(ref e) if e.kind() == io::ErrorKind::WouldBlock => {
                continue;
            }
            Err(e) => {
                return Err(e.into());
            }
        };
    }
    Ok(())
}
