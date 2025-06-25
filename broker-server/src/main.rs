use crate::opts::Opts;

mod opts;
mod server;
mod tests;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    use clap::Parser;
    let opts = Opts::parse();

    opts.configure_logging();

    server::server::run(&opts).await
}
