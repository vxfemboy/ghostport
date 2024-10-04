use clap::Parser;
use rand::seq::SliceRandom;
use tokio::net::TcpListener;
use tokio::io::AsyncWriteExt;
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
    //debug!("Read {} signatures", signatures.len());

    let listener = TcpListener::bind(&cli.listen).await?;
    info!("Started listener on {}", cli.listen);

    loop {
        // Accept connection
        let (mut stream, address) = listener.accept().await?;
        if cli.debug {
            debug!("Accepted connection from {}", address);
        } else if cli.verbose {
            info!("Accepted connection from {}", address);
        }

        let sigs = signatures.clone();
        let cli_clone = cli.clone();

        // Spawn async thread
        tokio::spawn(async move {
            // Choose random signature
            let signature = sigs.choose(&mut rand::thread_rng());

            if let Some(sig) = signature {
                let payload = generate_payload(sig);

                // Write payload
                match stream.write_all(&payload).await {
                    Ok(()) => {
                        if cli_clone.debug {
                            debug!(
                                "Sent payload to {}: {:?} ({} bytes)",
                                address,
                                String::from_utf8_lossy(&payload),
                                payload.len()
                            );
                        } else if cli_clone.verbose {
                            info!("Sent payload ({} bytes) to {}", payload.len(), address);
                        }
                    }
                    Err(e) => {
                        if e.kind() == std::io::ErrorKind::ConnectionReset {
                            debug!("Connection reset by peer: {}", address);
                        } else {
                            error!("Failed to write payload to {}: {}", address, e);
                        }
                    }
                }
            } else {
                debug!("No signature available");
            }
        });
    }
}
