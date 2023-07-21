use clap::Parser;
use fs::{check_dir, check_file};
use log::info;
use std::{
    fs::OpenOptions,
    io::prelude::*,
    path::PathBuf,
    process::{self, Command, Stdio},
};

#[derive(Parser, Debug, Clone)]
pub struct ContiguousArgs {
    /// File to write queries to
    #[arg(short, long, default_value = concat!(env!("HOME"), "/.mina-indexer-contiguous-block-queries"))]
    query_file: PathBuf,
    /// Directory to dump blocks into
    #[arg(short, long, default_value = concat!(env!("HOME"), "/.mina-indexer-contiguous-blocks"))]
    blocks_dir: PathBuf,
    /// Start block blockchain_length
    #[arg(short, long, default_value_t = 2)]
    start: usize,
    /// Number of block lengths to download
    #[arg(short, long, default_value_t = 1000)]
    num: usize,
}

pub fn main(args: ContiguousArgs) -> anyhow::Result<()> {
    let query_file = args.query_file;
    let blocks_dir = args.blocks_dir;
    let start = args.start;
    let num = args.num;

    check_file(&query_file);
    check_dir(&blocks_dir);

    // check gsutil is installed
    match Command::new("gsutil").arg("version").output() {
        Ok(_) => (),
        Err(_) => {
            println!(
                "Please install gsutil! See https://cloud.google.com/storage/docs/gsutil_install"
            );
            process::exit(2);
        }
    }

    // write query file to download the desired Mina blocks
    let mut file = OpenOptions::new().append(true).open(query_file.clone())?;
    file.set_len(0)?;

    info!("Writing query file...");
    for height in start..(num + start) {
        writeln!(file, "gs://mina_network_block_data/mainnet-{height}-*.json")?;
    }

    // pass the file to gsutil -m cp -I
    let cat_cmd = Command::new("cat")
        .arg(query_file)
        .stdout(Stdio::piped())
        .spawn()
        .unwrap();

    let mut gsutil_cmd = Command::new("gsutil")
        .arg("-m")
        .arg("cp")
        .arg("-n")
        .arg("-I")
        .arg(blocks_dir)
        .stdin(Stdio::from(cat_cmd.stdout.unwrap()))
        .spawn()
        .unwrap();

    match gsutil_cmd.wait() {
        Ok(_) => Ok(()),
        Err(e) => Err(anyhow::Error::from(e)),
    }
}
