use clap::Parser;
use rand::seq::SliceRandom;
use tokio::net::TcpListener;
use tracing::{debug, error, info, Level};

mod cli;
mod handler;

use cli::Cli;
use handler::{generate_payload, parse_signatures};

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
    let signatures = match parse_signatures(&cli.signatures) {
        Ok(sigs) => sigs,
        Err(e) => {
            error!("Failed to parse signatures file: {}", e);
            return Err(e);
        }
    };
    debug!("Read {} signatures", signatures.len());

    // Bind listener
    let listener = TcpListener::bind(&cli.listen).await?;
    info!("Started listener on {}", cli.listen);

    loop {
        // Accept connection
        let (stream, address) = listener.accept().await?;
        if cli.debug {
            debug!("Accepted connection from {}", address);
        } else if cli.verbose {
            info!("Accepted connection from {}", address);
        }

        // Clone signatures
        let sigs = signatures.clone();

        // Spawn async thread
        tokio::spawn(async move {
            // Choose random signature
            let signature = sigs.choose(&mut rand::thread_rng());

            if let Some(sig) = signature {
                // Generate payload
                let payload = generate_payload(sig);

                // Write payload
                if let Err(e) = stream.try_write(&payload) {
                    error!("Failed to write payload to {}: {}", address, e);
                    return;
                }

                if cli.debug {
                    debug!(
                        "Sent payload to {}: {:?}",
                        address,
                        String::from_utf8_lossy(&payload)
                    );
                } else if cli.verbose {
                    info!("Sent payload to {}", address);
                }
            } else {
                debug!("No signature available");
            }
        });
    }
}
