use clap::Parser;
use std::path::PathBuf;

#[derive(Parser, Debug, Clone)]
pub struct SubcommandArgs {
    /// Path to blocks directory
    #[arg(short, long, default_value = concat!(env!("HOME"), "/.blocks"))]
    pub blocks_dir: PathBuf,
    /// File to output the list of canonical blocks
    #[arg(short, long, default_value = concat!(env!("HOME"), "/.output"))]
    pub output_file: PathBuf,
}

pub const BLOCK_REPORTING_FREQ: u32 = 5000;
pub const MAINNET_CANONICAL_THRESHOLD: u32 = 10;
