use broker_server::opts::Opts;
use broker_server::server;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    use clap::Parser;
    let opts = Opts::parse();

    opts.configure_logging();

    server::server::run(&opts).await
}
