use tokio::net::TcpStream;
use tokio::io::{AsyncReadExt, AsyncWriteExt, BufReader, BufWriter};
use common::protocol::Message;

pub struct Connection {
    reader: BufReader<tokio::net::tcp::OwnedReadHalf>,
    writer: BufWriter<tokio::net::tcp::OwnedWriteHalf>,
}

impl Connection {
    pub fn new(stream: TcpStream) -> Self {
        let (read_half, write_half) = stream.into_split();
        Self {
            reader: BufReader::new(read_half),
            writer: BufWriter::new(write_half),
        }
    }

    pub async fn send(&mut self, msg: Message) -> anyhow::Result<()> {
        let bytes = bincode::serialize(&msg)?;
        self.writer.write_all(&(bytes.len() as u32).to_be_bytes()).await?;
        self.writer.write_all(&bytes).await?;
        self.writer.flush().await?;
        Ok(())
    }

    pub async fn read_loop(&mut self) -> anyhow::Result<()> {
        loop {
            let mut len_buf = [0u8; 4];
            if self.reader.read_exact(&mut len_buf).await.is_err() {
                break;
            }
            let len = u32::from_be_bytes(len_buf);
            let mut buf = vec![0u8; len as usize];
            self.reader.read_exact(&mut buf).await?;
            let msg: Message = bincode::deserialize(&buf)?;
            println!("[client] Received: {:?}", msg);
        }
        Ok(())
    }
}
