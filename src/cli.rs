use std::path::PathBuf;

use clap::Parser;

#[derive(Debug, Parser)]
#[command(author, version, about, long_about=None)]
pub struct Cli {
    /// The YEAR where the genetion end.
    pub year: usize,
    /// Path to the CONFIG toml file.
    #[arg(short, long, value_name = "CONFIG")]
    pub config: Option<PathBuf>,
}
