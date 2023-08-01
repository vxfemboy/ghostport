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
}
