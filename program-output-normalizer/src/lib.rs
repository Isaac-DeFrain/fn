use clap::Parser;

pub mod config;
pub mod error;
pub mod version;

#[derive(Parser, Debug)]
pub struct Program {
    /// Version of config file
    #[arg(short, long)]
    pub version: version::Version,
    /// Config type
    #[arg(long)]
    pub config: String,
    /// Command to normalize the output
    #[arg(short, long)]
    pub command: String,
}
