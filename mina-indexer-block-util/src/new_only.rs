use blockchain::*;
use clap::Parser;
use fs::check_file;
use glob::glob;
use log::{debug, info};
use std::{
    fs::{read_to_string, File, OpenOptions},
    io::prelude::*,
    path::PathBuf,
    process::{self, Command, Stdio},
    str::FromStr,
};

#[derive(Parser, Debug, Clone)]
pub struct NewArgs {
    /// File to write queries to
    #[arg(short, long, default_value = concat!(env!("HOME"), "/.mina-indexer-new-block-queries"))]
    query_file: PathBuf,
    /// Directory to dump blocks into
    #[arg(short, long, default_value = concat!(env!("HOME"), "/.mina-indexer-new-blocks"))]
    blocks_dir: PathBuf,
    /// File to write gsutil ls to
    #[arg(short, long, default_value = concat!(env!("HOME"), "/.mina-indexer-ls"))]
    ls_file: PathBuf,
    /// The number of block lengths below the current max to query
    #[arg(long, default_value_t = 10)]
    buffer: u32,
    /// Start downloading all blocks at or above this length
    #[arg(short, long, default_value = None)]
    start: Option<u32>,
    /// Download strictly blocks strictly above the current max height
    #[arg(long, default_value_t = false)]
    strict: bool,
    /// Name of GCP bucket
    #[arg(long, default_value = "mina_network_block_data")]
    bucket: String,
    /// Name of Mina network
    #[arg(long, default_value = "mainnet")]
    network: String,
    /// Skip the ls file creation if you already have a substantial amount of blocks
    #[arg(short, long, default_value_t = false)]
    skip_ls_file: bool,
}

pub fn main(args: NewArgs) -> anyhow::Result<()> {
    let query_file_path = args.query_file;
    let blocks_dir = args.blocks_dir;
    let ls_file_path = args.ls_file;
    let buffer = args.buffer;
    let start = args.start;
    let strict = args.strict;
    let bucket = args.bucket;
    let network = args.network;
    let skip_ls_file = args.skip_ls_file;

    // check gsutil is installed
    match Command::new("gsutil").arg("version").spawn() {
        Ok(_) => (),
        Err(_) => {
            println!(
                "Please install gsutil! see https://cloud.google.com/storage/docs/gsutil_install"
            );
            process::exit(2);
        }
    }

    check_file(&query_file_path);
    assert!(blocks_dir.exists(), "Must supply a blocks dir!");
    assert!(!strict || start.is_none(), "Can't use `--start` and `--strict` together");

    info!("Reading block directory {}", blocks_dir.display());

    // get max length from blocks in blocks_dir
    let mut our_block_paths: Vec<PathBuf> =
        glob(&format!("{}/{network}-*-*.json", blocks_dir.display()))
            .unwrap()
            .filter_map(|p| p.ok())
            .collect();

    our_block_paths.sort_by(|x, y| {
        length_from_path(x)
            .unwrap()
            .cmp(&length_from_path(y).unwrap())
    });
    let our_max_length = MinaMainnetBlock::from_str(
        our_block_paths
            .last()
            .unwrap()
            .file_name()
            .unwrap()
            .to_str()
            .unwrap(),
    )?
    .length;

    info!("Our max {network} block length: {our_max_length}");
    info!(
        "Max length {network} block retrieved {:?}m ago",
        our_block_paths
            .last()
            .unwrap()
            .metadata()
            .unwrap()
            .created()
            .unwrap()
            .elapsed()
            .unwrap()
            .as_secs_f32()
            / 60_f32
    );

    let ls_file;
    if skip_ls_file || query_file_path.exists() {
        if skip_ls_file {
            info!("ls file creation skipped");
        } else {
            info!("ls file found - searching for blocks since last modification");
        }

        ls_file = File::open(ls_file_path.clone())?;
        let min_since_modified = ls_file
            .metadata()
            .unwrap()
            .modified()
            .unwrap()
            .elapsed()
            .unwrap()
            .as_secs() as f32
            / 60_f32;

        if !skip_ls_file {
            info!("{min_since_modified} min since last modification");
            info!(
                "Potentially {} new {network} block lengths",
                min_since_modified as u32 / 3
            );
        }

        let mut query_file = File::create(query_file_path.clone()).unwrap();
        let max_network_length = our_max_length + (min_since_modified as u32 / 3) + 1;

            // write query file with appropriate URIs
            debug!("Writing query file: {}", query_file_path.display());
            let start = match (strict, start) {
                (true, None) => 2.max(our_max_length + 1),
                (false, None) => if buffer < our_max_length {
                    2.max(our_max_length - buffer)
                } else {
                    2
                },
                (false, Some(start_length)) => start_length,
                _ => unreachable!(),
            };
            for length in start..=max_network_length {
                writeln!(query_file, "gs://{bucket}/{network}-{length}-*.json")?;
            }
        info!(
            "Querying {network} block lengths: {}..{max_network_length}",
            2.max(our_max_length - 10)
        );
    } else {
        info!("Querying all {network} blocks from {bucket}. This may take a while...");

        // ls all mainnet blocks with length from mina_network_block_data bucket, collect in vec
        ls_file = File::create(ls_file_path.clone())?;
        let mut gsutil_ls_cmd = Command::new("gsutil")
            .arg("-m")
            .arg("ls")
            .arg(&format!("gs://{bucket}/{network}-*-*.json"))
            .stdout(Stdio::from(ls_file))
            .spawn()
            .unwrap();

        match gsutil_ls_cmd.wait() {
            Ok(_) => (),
            Err(e) => return Err(anyhow::Error::from(e)),
        }

        let mut all_network_blocks: Vec<MinaBlockQuery> = read_to_string(&ls_file_path)?
            .lines()
            .filter_map(|q| MinaBlockQuery::from_str(q).ok())
            .collect();

        info!(
            "{} {network} blocks found in bucket",
            all_network_blocks.len()
        );
        all_network_blocks.sort_by(|x, y| x.length.cmp(&y.length));

        let max_network_length = all_network_blocks.last().map_or(0, |q| q.length);
        info!("{network} max block length: {max_network_length}");

        // start at our current max length - 10
        let mut query_file = File::create(query_file_path.clone())?;
        for query in all_network_blocks
            .iter()
            .skip_while(|q| q.length < our_max_length - 10)
        {
            writeln!(query_file, "{}", query.to_string())?;
        }
    }

    // download the blocks
    // `cat query_file | gsutil -m cp -I blocks_dir`
    let cat_cmd = Command::new("cat")
        .arg(query_file_path.clone())
        .stdout(Stdio::piped())
        .spawn()
        .unwrap();

    let mut gsutil_cp_cmd = Command::new("gsutil")
        .arg("-m")
        .arg("cp")
        .arg("-I")
        .arg(blocks_dir)
        .stdin(Stdio::from(cat_cmd.stdout.unwrap()))
        .spawn()
        .unwrap();

    match gsutil_cp_cmd.wait() {
        Ok(exit_status) => println!("{exit_status}"),
        Err(e) => return Err(anyhow::Error::from(e)),
    }

    // clear & keep ls file, remove query file
    let mut ls_file = OpenOptions::new().write(true).open(ls_file_path).unwrap();
    write!(ls_file, "")?;
    std::fs::remove_file(query_file_path)?;

    Ok(())
}

struct MinaBlockQuery {
    length: u32,
    state_hash: String,
    bucket: String,
    network: String,
}

impl FromStr for MinaBlockQuery {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        // shape = gs://bucket/network-length-state_hash.json
        if let Some(all_fields) = s.strip_prefix("gs://") {
            let mut all_fields = all_fields.split('/');
            let bucket = all_fields.next().unwrap().to_string();
            let network_length_hash = all_fields.next().unwrap();
            let mut parts = network_length_hash.split('-');
            let network = parts.next().unwrap().to_string();
            let length: u32 = parts.next().unwrap().parse()?;
            let state_hash = parts.next().unwrap().split('.').next().unwrap().to_string();

            return Ok(MinaBlockQuery {
                length,
                state_hash,
                bucket,
                network,
            });
        }
        Err(anyhow::Error::msg(format!("{s} parsed incorrectly!")))
    }
}

impl ToString for MinaBlockQuery {
    fn to_string(&self) -> String {
        format!(
            "gs://{}/{}-{}-{}.json",
            self.bucket, self.network, self.length, self.state_hash
        )
    }
}

struct MinaMainnetBlock {
    length: u32,
}

impl FromStr for MinaMainnetBlock {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        if let Some(length_and_hash) = s.strip_prefix("*-") {
            let length: u32 = length_and_hash.split('-').next().unwrap().parse()?;

            return Ok(MinaMainnetBlock { length });
        }
        Err(anyhow::Error::msg(format!("{s} parsed incorrectly!")))
    }
}
