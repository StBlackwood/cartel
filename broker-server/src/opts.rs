use clap::Parser;
use serde::{Deserialize, Serialize};
use tracing::Level;
use tracing_subscriber::filter::filter_fn;
use tracing_subscriber::layer::SubscriberExt;
use tracing_subscriber::util::SubscriberInitExt;
use tracing_subscriber::Layer;

pub const DEFAULT_SERVER_ADDRESS: &str = "127.0.0.1";
pub const DEFAULT_SERVER_PORT: u16 = 9802;

#[derive(Debug, Serialize, Deserialize, Parser)]
pub struct Opts {
    #[arg(long, default_value_t = String::from(DEFAULT_SERVER_ADDRESS))]
    pub host: String,

    #[arg(long, default_value_t = DEFAULT_SERVER_PORT)]
    pub port: u16,

    #[arg(short, long, default_value_t = verbosity_default())]
    pub verbose: u8,
}
impl Opts {
    pub fn configure_logging(&self) {
        let loglevel = match self.verbose {
            1 => Level::DEBUG,
            2 => Level::TRACE,
            _ => Level::INFO,
        };

        let prefix_blacklist = &["netlink", "tokio_util", "aws", "rustls", "hyper", "mio"];

        tracing_subscriber::registry()
            .with(
                tracing_subscriber::fmt::layer().with_filter(filter_fn(move |metadata| {
                    !prefix_blacklist
                        .iter()
                        .any(|prefix| metadata.target().starts_with(prefix))
                        && metadata.level() <= &loglevel
                })),
            )
            .init();
    }
}

fn verbosity_default() -> u8 {
    if cfg!(debug_assertions) {
        2
    } else {
        0
    }
}
