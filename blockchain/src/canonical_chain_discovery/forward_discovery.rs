use super::common::*;
use fs::{check_dir, check_file};
use glob::glob;
use log::{error, info};
use std::{
    collections::HashSet, fs::OpenOptions, io::Write, path::PathBuf, process, time::Instant,
    u32::MAX,
};

pub fn main(args: &SubcommandArgs) -> anyhow::Result<()> {
    let blocks_dir = args.blocks_dir.clone();
    let output_file_path = args.output_file.clone();

    check_dir(&blocks_dir);
    check_file(&output_file_path);

    let mut output_file = OpenOptions::new().append(true).open(&output_file_path)?;

    if blocks_dir.exists() && blocks_dir.is_dir() {
        let total = Instant::now();

        info!("Collectiong block paths...");
        let mut paths: Vec<PathBuf> = glob(&format!("{}/*.json", blocks_dir.display()))
            .expect("Failed to read glob pattern")
            .filter_map(|x| x.ok())
            .collect();
        info!("Collection took {:?}", total.elapsed());

        let mut canonical_paths = vec![];
        let mut successive_paths = vec![];

        if paths.is_empty() {
            error!("No blocks found in {}", blocks_dir.display());
            process::exit(1);
        } else {
            info!("Sorting blocks by length...");

            let time = Instant::now();
            paths.sort_by(|x, y| {
                length_from_path(x)
                    .unwrap_or(MAX)
                    .cmp(&length_from_path(y).unwrap_or(MAX))
            });

            info!("Sorted {} blocks in {:?}", paths.len(), time.elapsed());
            info!("Searching for canonical chain...");

            let mut length_start_indices = vec![];
            let mut curr_length = length_from_path(paths.first().unwrap()).unwrap();

            // build the length_start_indices vec corresponding to the
            // longest contiguous chain starting from the lowest block
            for (idx, path) in paths.iter().enumerate() {
                let height = length_from_path(path).unwrap_or(MAX);
                if idx == 0 || height > curr_length {
                    length_start_indices.push(idx);
                    curr_length = height; // this is curr_length + 1 except for the blocks with no explicit height
                } else {
                    continue;
                }
            }

            // prune non-canonical blocks
            // a block is definitely canonical if there exists a (sub)chain of length 10 on top of it
            let mut length_idx = 0;
            let mut hash_stash = HashSet::with_capacity(10);

            let path = paths
                .get(*length_start_indices.get(length_idx).unwrap())
                .unwrap();
            let mut curr_length = length_from_path(path);

            // check that there are sufficiently many blocks ahead
            // if not, we exit the loop
            while length_start_indices
                .get(length_idx + MAINNET_CANONICAL_THRESHOLD as usize)
                .is_some()
            {
                let count = canonical_paths.len();
                if count > 0 && (count * 5) % BLOCK_REPORTING_FREQ as usize == 0 {
                    info!("Found {count} canonical blocks in {:?}", time.elapsed());
                }

                // collect blocks at curr_length
                let mut at_same_length = vec![];
                for path in paths[length_idx..].iter() {
                    if length_from_path(path) == curr_length {
                        at_same_length.push(path);
                    } else {
                        continue;
                    }
                }

                length_idx += 1;

                // check for sufficiently long hash-linked chain ahead
                for path in at_same_length {
                    let next_length_idx = length_start_indices.get(length_idx).unwrap();
                    let mut next_length_idx_offset = 0;

                    for n in 1..=10 {
                        // start at the height + n blocks
                        let mut next_length = curr_length.map(|h| h + n);
                        let mut curr_hash = hash_from_path(path).unwrap();

                        while paths
                            .get(next_length_idx + next_length_idx_offset)
                            .map(|p| length_from_path(p).unwrap_or(MAX))
                            == next_length
                        {
                            let path = paths.get(next_length_idx + next_length_idx_offset).unwrap();

                            next_length_idx_offset += 1;
                            let parent_hash = extract_parent_hash_from_path(path).unwrap();

                            // if the block is a descendant, add the hash and proceed to the next block
                            if parent_hash == curr_hash {
                                curr_hash = parent_hash.clone();
                                hash_stash.insert(parent_hash);

                                if n == 10 {
                                    for path in paths.iter().filter(|p| {
                                        hash_stash.contains(&hash_from_path(p).unwrap())
                                    }) {
                                        canonical_paths
                                            .push(path.file_name().unwrap().to_str().unwrap());
                                    }
                                    continue;
                                } else {
                                    // if the block has an nth descendant, we need to check if it has an (n + 1)th descendant
                                    // to do this, we need to find the first block of height + n, update next_length and idx_offset
                                    next_length = curr_length.map(|h| h + 1);
                                }
                            } else {
                                continue;
                            }
                        }
                    }
                }
                curr_length = curr_length.map(|h| h + 1);
            }

            for path in paths[length_start_indices[length_idx + 1]..].iter() {
                successive_paths.push(path.file_name().unwrap().to_str().unwrap());
            }
        }

        let time = Instant::now();

        // write the canonical block files
        for canonical in &canonical_paths {
            writeln!(output_file, "{canonical}")?;
        }

        // write the successive block files
        for successive in &successive_paths {
            writeln!(output_file, "{successive}")?;
        }

        output_file.flush()?;
        info!(
            "{} written to {} in {:?}",
            canonical_paths.len() + successive_paths.len(),
            output_file_path.display(),
            time.elapsed()
        );
        info!("Total time: {:?}", total.elapsed());

        Ok(())
    } else {
        Err(anyhow::Error::msg(format!(
            "Blocks dir {} must be a directory",
            blocks_dir.display()
        )))
    }
}
