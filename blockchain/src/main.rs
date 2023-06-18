use std::{
    collections::HashSet,
    fs::File,    
    io::{prelude::*, SeekFrom},
    path::{PathBuf, Path},
    u32::MAX, ffi::OsStr,
};
use clap::Parser;
use glob::glob;
use log::{debug, info};

#[derive(Debug, Clone, Parser)]
struct Args {
    /// Path to blocks directory
    #[arg(short, long)]
    blocks_dir: PathBuf
}

fn main() -> anyhow::Result<()> {
    let args = Args::parse();
    let blocks_dir = args.blocks_dir;

    if blocks_dir.exists() {
        // filename-ordered block paths
        let mut paths: Vec<PathBuf> = glob(&format!("{}/*.json", blocks_dir.display()))
            .expect("Failed to read glob pattern")
            .filter_map(|x| x.ok())
            .collect();

        let mut canonical = vec![];
        if paths.len() != 0 {
            info!("Sorting start up blocks by height");
            paths.sort_by(|x, y| {
                length_from_path(x)
                    .unwrap_or(MAX)
                    .cmp(&length_from_path(y).unwrap_or(MAX))
            });

            info!("Searching for canonical chain in startup blocks");
            let mut height_start_indices = vec![];
            let mut curr_height = length_from_path(&paths.first().unwrap()).unwrap();

            for (idx, path) in paths.iter().enumerate() {
                let height = length_from_path(path).unwrap_or(MAX);
                if height > curr_height {
                    height_start_indices.push(idx);
                    curr_height = height; // this is curr_height + 1 except for the blocks with no explicit height
                }
            }

            // prune non-canonical blocks
            // a block is definitely canonical if there exists a (sub)chain of length 10 on top of it
            let mut height_idx = 0;
            let mut hash_stash = HashSet::with_capacity(10);

            let path = paths.get(*height_start_indices.get(height_idx).unwrap()).unwrap();
            let mut curr_height = length_from_path(path);

            // check that there are sufficiently many blocks ahead
            // if not, we exit the loop
            while height_start_indices.get(height_idx + 11).is_some() {
                // collect blocks at curr_height
                let mut at_same_height = vec![];
                for path in paths[height_idx..].iter() {
                    if length_from_path(path) == curr_height {
                        at_same_height.push(path);
                        debug!("Push path: {}", path.display());
                    } else {
                        continue;
                    }
                }

                height_idx += 1;

                // check for sufficiently long hash-linked chain ahead
                for path in at_same_height {
                    let next_height_idx = height_start_indices.get(height_idx).unwrap();
                    debug!("Checking path: {}", path.display());
                    let mut next_height_idx_offset = 0;

                    for n in 1..=10 {
                        debug!("Current height: {}, n: {n}", curr_height.unwrap());
                        // start at the height + n blocks
                        let mut next_height = curr_height.map(|h| h + n);
                        let mut curr_hash = hash_from_path(path).unwrap();

                        while paths
                            .get(next_height_idx + next_height_idx_offset)
                            .map(|p| length_from_path(p).unwrap_or(MAX))
                            == next_height
                        {
                            let path =
                                paths.get(next_height_idx + next_height_idx_offset).unwrap();
                            debug!("{}", path.display());

                            next_height_idx_offset += 1;
                            let parent_hash = extract_parent_hash_from_path(path).unwrap();
                            debug!("Parent hash: {parent_hash}");

                            // if the block is a descendant, add the hash and proceed to the next block
                            if &parent_hash == &curr_hash {
                                curr_hash = parent_hash.clone();
                                hash_stash.insert(parent_hash);

                                if n == 10 {
                                    for path in paths.iter().filter(|p| {
                                        hash_stash.contains(&hash_from_path(p).unwrap())
                                    }) {
                                        canonical.push(path.clone());
                                        debug!("Added path {}", path.display());
                                    }
                                    break;
                                } else {
                                    // if the block has an nth descendant, we need to check if it has an (n + 1)th descendant
                                    // to do this, we need to find the first block of height + n, update next_height and idx_offset
                                    next_height = curr_height.map(|h| h + 1);
                                }
                            } else {
                                continue;
                            }
                        }
                    }
                }
                curr_height = curr_height.map(|h| h + 1);
            }
        }

        for path in canonical {
            info!("{}", path.display());
        }

        Ok(())
    } else {
        Err(anyhow::Error::msg(format!("Blocks dir {} must be a directory", blocks_dir.display())))
    }
}

fn length_from_path(path: &Path) -> Option<u32> {
    get_blockchain_length(path.file_name().unwrap())
}

fn hash_from_path(path: &Path) -> Option<String> {
    get_state_hash(path.file_name().unwrap())
}

fn extract_parent_hash_from_path(path: &Path) -> anyhow::Result<String> {
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
fn get_state_hash(file_name: &OsStr) -> Option<String> {
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
fn get_blockchain_length(file_name: &OsStr) -> Option<u32> {
    file_name
        .to_str()?
        .split('-')
        .fold(None, |acc, x| match x.parse::<u32>() {
            Err(_) => acc,
            Ok(x) => Some(x),
        })
}
