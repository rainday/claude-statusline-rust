use clap::Parser;

#[derive(Parser, Debug)]
#[command(name = "claude-statusline-rs", about = "Custom statusline for Claude Code")]
pub struct Cli {
    /// Print version
    #[arg(short, long)]
    pub version: bool,

    /// Path to config file
    #[arg(short, long)]
    pub config: Option<String>,
}

impl Cli {
    pub fn parse_args() -> Self {
        Cli::parse()
    }
}
