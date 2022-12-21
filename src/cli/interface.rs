use clap::{Parser, ValueEnum};

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
pub struct Cli {
    #[arg(value_enum)]
    pub mode: Mode,
}

#[derive(Debug, Copy, Clone, Ord, PartialOrd, PartialEq, Eq, ValueEnum)]
pub enum Mode {
    Watch,
    List,
    Version,
}

/**
 * Command line arguments.
 */
pub fn args() -> Cli {
    Cli::parse()
}
