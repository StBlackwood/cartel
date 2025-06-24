use tokio::net::TcpStream;
use tokio::io::AsyncWriteExt;

pub async fn handle_connection(mut stream: TcpStream) -> anyhow::Result<()> {
    let greeting = b"Hello from server!\n";
    stream.write_all(greeting).await?;
    stream.flush().await?;
    Ok(())
}
