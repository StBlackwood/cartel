use crate::connection::handle_connection;
use crate::opts::Opts;
use tokio::net::TcpListener;
use tracing::{info};

pub async fn run(opts: &Opts) -> anyhow::Result<()> {
    let addr = format!("{}:{}", opts.host, opts.port);
    let listener = TcpListener::bind(&addr).await?;
    info!("Broker listening on {}", addr);

    loop {
        let (socket, peer) = listener.accept().await?;
        info!("New connection from {}", peer);
        tokio::spawn(handle_connection(socket));
    }
}
