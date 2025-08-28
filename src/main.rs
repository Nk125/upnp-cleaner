use igd_next::SearchOptions;
use igd_next::aio::tokio::search_gateway;
use upnp_cleaner::args::{self, Parser};

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let parsed_args = args::Args::parse();

    let options = SearchOptions::default();

    println!("Searching gateway...");

    let gateway = search_gateway(options).await?;

    println!(
        "Removing port: {}, protocol: {}",
        parsed_args.port, parsed_args.net_protocol
    );

    gateway
        .remove_port(parsed_args.net_protocol.into(), parsed_args.port)
        .await?;

    println!("Port unmapped successfully!");

    Ok(())
}
