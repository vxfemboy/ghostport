use clap::Parser;

#[derive(Parser, Debug)]
#[command(author = "perp")]
// CLI flags/arguments
pub struct Cli {
    #[arg(
        short = 's',
        long = "signatures",
        value_name = "FILE",
        help = "Path to the signatures",
        default_value = "signatures"
    )]
    pub signatures: String,

    #[arg(
        short = 'l',
        long = "listen",
        value_name = "ADDRESS",
        help = "Address to listen on",
        default_value = "127.0.0.1:8888"
    )]
    pub listen: String,

    #[arg(
        short = 'd',
        long = "debug",
        help = "Enable debug logging"
    )]
    pub debug: bool,

    #[arg(
        short = 'v',
        long = "verbose",
        help = "Enable verbose logging"
    )]
    pub verbose: bool,

    #[arg(
        short = 'q',
        long = "quiet",
        help = "Enable quiet logging"
    )]
    pub quiet: bool,

    #[arg(
        short = 'V',
        long = "version",
        help = "Print version information"
    )]
    pub version: bool,
}
