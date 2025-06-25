use tokio::net::TcpStream;
use tokio::io::{AsyncReadExt, AsyncWriteExt};
use common::protocol::Message;

pub async fn handle_connection(mut stream: TcpStream) -> anyhow::Result<()> {
    let mut len_buf = [0u8; 4];
    stream.read_exact(&mut len_buf).await?;
    let msg_len = u32::from_be_bytes(len_buf) as usize;

    let mut buf = vec![0u8; msg_len];
    stream.read_exact(&mut buf).await?;

    let msg: Message = bincode::deserialize(&buf)?;

    match msg {
        Message::Handshake { client_id, role } => {
            println!("Client {} connected as {:?}", client_id, role);
            let ack = Message::Ack;
            let encoded = bincode::serialize(&ack)?;
            stream.write_all(&(encoded.len() as u32).to_be_bytes()).await?;
            stream.write_all(&encoded).await?;
        }
        _ => {
            // handle other message types
        }
    }

    Ok(())
}
