use clap::Parser;
use rand::seq::SliceRandom;
use tokio::net::TcpListener;
use tracing::{debug, info, Level};

use cli::Cli;

mod cli;
mod config;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    // Setup logger
    tracing_subscriber::fmt()
        .without_time()
        .with_max_level(Level::DEBUG)
        .init();

    // Parse CLI
    let cli = Cli::parse();
    debug!("Parsed CLI flags");
    // Read signatures file
    let signatures = config::read_signatures(&cli.signatures)?;
    debug!("Read signatures file");

    // Bind listener
    let listener = TcpListener::bind("127.0.0.1:8888").await?;
    info!("Started listener");

    loop {
        // Accept connection
        let (stream, address) = listener.accept().await?;
        debug!("Accepted connection");
        // Clone signatures
        let sigs = signatures.clone();

        // Spawn async thread
        tokio::spawn(async move {
            // Choose random signature
            let signature = sigs.choose(&mut rand::thread_rng());

            // Write signature
            match stream.try_write(signature.expect("could not send signature").as_bytes()) {
                Ok(n) => {
                    debug!("Sent signature {:?} to {}", signature, address);
                    n
                }
                Err(_) => return,
            };
        });
    }
}
