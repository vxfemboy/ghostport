use clap::Parser;

#[derive(Parser, Debug)]
#[command(author = "perp")]
// CLI flags/arguments
pub struct Cli {
    #[arg(
        short = 'c',
        long = "config",
        value_name = "FILE",
        help = "Path to the config",
        default_value = "~/.config/ghostport/config.toml"
    )]
    pub config: String,
}
