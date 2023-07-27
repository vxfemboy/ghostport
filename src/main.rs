use clap::Parser;
use tokio::net::TcpListener;

use cli::Cli;

mod cli;
mod config;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    // Parse cli & read config
    let cli = Cli::parse();
    let config = config::read(&cli.config)?;

    // Bind listener
    let listener = TcpListener::bind("127.0.0.1:8888").await?;

    loop {
        // Accept connection
        let (stream, _) = listener.accept().await?;
        let config = config.clone();

        // Spawn async thread
        tokio::spawn(async move {
            // Go through payload line by line
            for line in config.payloads {
                // Format payload & write it
                let payload = format!("{}\r\n", line);
                match stream.try_write(payload.as_bytes()) {
                    Ok(n) => n,
                    Err(_) => return,
                };
            }
        });
    }
}
