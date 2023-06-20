use clap::Parser;
use std::{
    fs::create_dir_all,
    path::PathBuf,
    process::{Command, Stdio},
};

#[derive(Parser, Debug, Clone)]
struct CliArgs {
    /// File to write queries to
    #[arg(short, long, default_value = concat!(env!("HOME"), "/.gsutil-mina-block-queries"))]
    query_file: PathBuf,
    /// Directory to dump blocks into
    #[arg(short, long, default_value = concat!(env!("HOME"), "/.gsutil-mina-blocks"))]
    blocks_dir: PathBuf,
    /// Start block blockchain_length
    #[arg(short, long, default_value_t = 2)]
    start: usize,
    /// Number of block lengths to download
    #[arg(short, long, default_value_t = 1000)]
    num: usize,
}

fn main() -> anyhow::Result<()> {
    let args = CliArgs::parse();
    let query_file = args.query_file;
    let blocks_dir = args.blocks_dir;
    let start = args.start;
    let num = args.num;

    check_file(&query_file);
    check_dir(&blocks_dir);

    // write a file to download the desired Mina blocks
    let mut contents = String::new();
    for height in start..(num + start) {
        contents.push_str(&format!(
            "gs://mina_network_block_data/mainnet-{height}-*.json\n"
        ));
    }

    std::fs::write(&query_file, contents).expect("File write failed");

    // pass the file to gsutil -m cp -I
    let cat_cmd = Command::new("cat")
        .arg(query_file)
        .stdout(Stdio::piped())
        .spawn()
        .unwrap();

    let mut gsutil_cmd = Command::new("gsutil")
        .arg("-m")
        .arg("cp")
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

fn check_file(file: &PathBuf) {
    if file.exists() {
        assert!(file.is_file(), "{} must be a file!", file.display());
    } else {
        create_dir_all(file.parent().unwrap()).unwrap();
    }
}

fn check_dir(dir: &PathBuf) {
    if dir.exists() {
        assert!(dir.is_dir(), "{} must be a dir!", dir.display());
    } else {
        create_dir_all(dir).unwrap()
    }
}
