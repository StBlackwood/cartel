use tokio::io::AsyncReadExt;
use tokio::net::TcpStream;

pub async fn connect_and_read(addr: &str) -> anyhow::Result<String> {
    let mut stream = TcpStream::connect(addr).await?;

    let mut buf = vec![0u8; 1024];
    let n = stream.read(&mut buf).await?;
    let msg = String::from_utf8_lossy(&buf[..n]).to_string();

    Ok(msg)
}
