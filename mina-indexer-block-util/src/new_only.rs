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
}

pub fn main(args: NewArgs) -> anyhow::Result<()> {
    let query_file = args.query_file;
    let blocks_dir = args.blocks_dir;
    let ls_file_path = args.ls_file;
    let buffer = args.buffer;
    let start = args.start;
    let strict = args.strict;

    assert!(blocks_dir.exists(), "Must supply a blocks dir!");
    assert!(!strict || start.is_none(), "Can't use `--start` and `--strict` together");

    info!("Reading block directory {}", blocks_dir.display());

    // get max length from blocks in blocks_dir
    let mut our_block_paths: Vec<PathBuf> =
        glob(&format!("{}/mainnet-*-*.json", blocks_dir.display()))
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

    info!("Our max block length: {our_max_length}");
    info!(
        "Max length block retrieved {:?}m ago",
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

    if query_file.exists() {
        assert!(query_file.is_file(), "Query file must be a file!");
        info!("Query file found");

        let min_since_modified = query_file
            .metadata()
            .unwrap()
            .modified()
            .unwrap()
            .elapsed()
            .unwrap()
            .as_secs() as f32
            / 60_f32;

        if min_since_modified > 3_f32 {
            info!("{min_since_modified} min since last modification");
            info!(
                "Potentially {} new block lengths",
                min_since_modified as u32 / 3
            );

            let mut file = File::create(query_file.clone()).unwrap();
            let max_mainnet_length = our_max_length + (min_since_modified as u32 / 3) + 1;

            for n in our_max_length..=max_mainnet_length {
                writeln!(file, "gs://mina_network_block_data/mainnet-{n}-*.json")?;
            }

            // write file with appropriate URIs
            let mut file = OpenOptions::new()
                .append(true)
                .open(query_file.clone())
                .unwrap();

            // query previous block lengths because we might have missed some
            debug!("Writing query file: {}", query_file.display());
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
            for length in start..=max_mainnet_length {
                writeln!(file, "gs://mina_network_block_data/mainnet-{length}-*.json")?;
            }
        } else {
            info!("Only {min_since_modified} min since last modification");
            process::exit(0);
        }
    } else {
        check_file(&query_file);
        info!("Querying mina_network_block_data bucket...");

        // ls all mainnet blocks with length from mina_network_block_data gcloud, collect in vec
        let ls_file = File::create(ls_file_path.clone())?;
        let mut gsutil_ls_cmd = Command::new("gsutil")
            .arg("-m")
            .arg("ls")
            .arg("gs://mina_network_block_data/mainnet-*-*.json")
            .stdout(Stdio::from(ls_file))
            .spawn()
            .unwrap();

        match gsutil_ls_cmd.wait() {
            Ok(_) => (),
            Err(e) => return Err(anyhow::Error::from(e)),
        }

        let mut new_mainnet_blocks: Vec<MinaMainnetBlockQuery> = read_to_string(&ls_file_path)?
            .lines()
            .filter_map(|q| MinaMainnetBlockQuery::from_str(q).ok())
            .collect();

        info!(
            "{} mainnet blocks found in bucket",
            new_mainnet_blocks.len()
        );
        new_mainnet_blocks.sort_by(|x, y| x.length.cmp(&y.length));

        let max_mainnet_length = new_mainnet_blocks.last().map_or(0, |q| q.length);
        info!("Mainnet max block length: {max_mainnet_length}");

        std::fs::copy(&ls_file_path, &query_file)?;
        std::fs::remove_file(ls_file_path)?;
    }

    // download the blocks
    let cat_cmd = Command::new("cat")
        .arg(query_file.clone())
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

    // clear query file
    OpenOptions::new().write(true).open(query_file).unwrap();

    Ok(())
}

struct MinaMainnetBlockQuery {
    length: u32,
    state_hash: String,
}

impl FromStr for MinaMainnetBlockQuery {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        if let Some(length_and_hash) = s.strip_prefix("gs://mina_network_block_data/mainnet-") {
            let mut parts = length_and_hash.split('-');
            let length: u32 = parts.next().unwrap().parse()?;
            let state_hash = parts.next().unwrap().split('.').next().unwrap().to_string();

            return Ok(MinaMainnetBlockQuery { length, state_hash });
        }
        Err(anyhow::Error::msg(format!("{s} parsed incorrectly!")))
    }
}

impl ToString for MinaMainnetBlockQuery {
    fn to_string(&self) -> String {
        format!(
            "gs://mina_network_block_data/mainnet-{}-{}.json",
            self.length, self.state_hash
        )
    }
}

struct MinaMainnetBlock {
    length: u32,
}

impl FromStr for MinaMainnetBlock {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        if let Some(length_and_hash) = s.strip_prefix("mainnet-") {
            let length: u32 = length_and_hash.split('-').next().unwrap().parse()?;

            return Ok(MinaMainnetBlock { length });
        }
        Err(anyhow::Error::msg(format!("{s} parsed incorrectly!")))
    }
}
