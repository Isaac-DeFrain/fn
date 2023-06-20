use clap::Parser;
use std::{
    ffi::OsStr,
    fs::File,
    io::{prelude::*, SeekFrom},
    path::{Path, PathBuf},
};

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

pub fn length_from_path(path: &Path) -> Option<u32> {
    get_blockchain_length(path.file_name().unwrap())
}

pub fn hash_from_path(path: &Path) -> Option<String> {
    get_state_hash(path.file_name().unwrap())
}

pub fn extract_parent_hash_from_path(path: &Path) -> anyhow::Result<String> {
    let parent_hash_offset = 75;
    let parent_hash_length = 52;

    let mut f = File::open(path)?;
    f.seek(SeekFrom::Start(parent_hash_offset))?;
    let mut buf = vec![0; parent_hash_length];
    f.read_exact(&mut buf)?;
    let parent_hash = String::from_utf8(buf)?;
    Ok(parent_hash)
}

/// extract a state hash from an OS file name
pub fn get_state_hash(file_name: &OsStr) -> Option<String> {
    let last_part = file_name.to_str()?.split('-').last()?.to_string();
    if last_part.starts_with('.') {
        return None;
    }
    if !last_part.starts_with("3N") {
        return None;
    }
    let state_hash = last_part.split('.').next()?;
    if state_hash.contains('.') {
        return None;
    }
    Some(state_hash.to_string())
}

/// extract a blockchain length from an OS file name
pub fn get_blockchain_length(file_name: &OsStr) -> Option<u32> {
    file_name
        .to_str()?
        .split('-')
        .fold(None, |acc, x| match x.parse::<u32>() {
            Err(_) => acc,
            Ok(x) => Some(x),
        })
}
