use tokio::net::TcpStream;
use common::protocol::{ClientRole, Message};
use crate::connection::Connection;

pub struct Client {
    pub id: String,
    pub role: ClientRole,
    connection: Connection,
}

impl Client {
    pub async fn connect(id: String, role: ClientRole, addr: &str) -> anyhow::Result<Self> {
        let stream = TcpStream::connect(addr).await?;
        let mut connection = Connection::new(stream);

        let handshake = Message::Handshake { client_id: id.clone(), role: role.clone() };
        connection.send(handshake).await?;

        Ok(Self { id, role, connection })
    }

    pub async fn send_message(&mut self, msg: Message) -> anyhow::Result<()> {
        self.connection.send(msg).await
    }

    pub async fn run_reader_loop(&mut self) -> anyhow::Result<()> {
        self.connection.read_loop().await
    }
}
