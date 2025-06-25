use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub enum Message {
    Handshake { client_id: String, role: ClientRole },
    Produce { topic: String, payload: Vec<u8> },
    Fetch { topic: String },
    Ack,
    Error { reason: String },
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum ClientRole {
    Producer,
    Consumer,
}
