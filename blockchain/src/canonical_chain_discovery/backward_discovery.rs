use super::common::*;
use fs::{check_dir, check_file};
use glob::glob;
use log::{error, info};
use std::{fs::OpenOptions, io::Write, path::PathBuf, process, time::Instant, u32::MAX};

pub fn main(args: &SubcommandArgs) -> anyhow::Result<()> {
    let blocks_dir = args.blocks_dir.clone();
    let output_file_path = args.output_file.clone();

    check_dir(&blocks_dir);
    check_file(&output_file_path);

    let mut output_file = OpenOptions::new().append(true).open(&output_file_path)?;

    if blocks_dir.exists() && blocks_dir.is_dir() {
        let total = Instant::now();

        info!("Collecting block paths...");
        let mut paths: Vec<PathBuf> = glob(&format!("{}/*.json", blocks_dir.display()))
            .expect("Failed to read glob pattern")
            .filter_map(|x| x.ok())
            .collect();
        info!("Collection took {:?}", total.elapsed());

        let mut successive_paths = vec![];
        let mut canonical_paths = vec![];

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
                    curr_length = height;
                } else {
                    continue;
                }
            }

            // check that there are enough contiguous blocks
            let check_lengths = length_start_indices
                .iter()
                .take(MAINNET_CANONICAL_THRESHOLD as usize + 1)
                .map(|idx| length_from_path(paths.get(*idx).unwrap()).unwrap_or(MAX));

            let checked = check_lengths.enumerate().fold(None, |acc, (n, x)| {
                if acc.is_none() && n == 0 || x == acc.unwrap_or(0) + 1 {
                    Some(x)
                } else {
                    None
                }
            });

            if checked.is_none() {
                error!("No canoncial blocks can be confidently found!");
                process::exit(1);
            }

            let (max_start_idx, max_length_idx) =
                if length_from_path(paths.last().unwrap()).is_some() {
                    (
                        length_start_indices.len() - 1,
                        *length_start_indices.last().unwrap(),
                    )
                } else {
                    (
                        length_start_indices.len() - 2,
                        length_start_indices[length_start_indices.len() - 2],
                    )
                };

            // backtrack canonical_threshold blocks to find a canonical one
            let mut curr_start_idx = max_start_idx;
            let mut curr_length_idx = max_length_idx;
            let mut curr_path = paths.get(curr_length_idx).unwrap();
            let time = Instant::now();

            for _ in 1..=MAINNET_CANONICAL_THRESHOLD {
                if curr_start_idx > 0 {
                    let prev_length_idx = length_start_indices[curr_start_idx - 1];

                    for path in paths[prev_length_idx..curr_length_idx].iter() {
                        if extract_parent_hash_from_path(curr_path).unwrap()
                            == hash_from_path(path).unwrap()
                        {
                            curr_path = path;
                            curr_length_idx = prev_length_idx;
                            curr_start_idx -= 1;
                            continue;
                        }
                    }
                }
            }

            let successive_idx = length_start_indices[curr_start_idx + 1];

            // curr_path represents a canonical block
            info!(
                "Found the canonical tip {} in {:?}",
                curr_path.display(),
                time.elapsed()
            );

            canonical_paths.push(curr_path.file_name().unwrap().to_str().unwrap());
            info!("Walking the canonical chain back to the beginning. Reporting every {BLOCK_REPORTING_FREQ} blocks.", );

            let time = Instant::now();
            while curr_start_idx > 0 {
                let count = canonical_paths.len();
                if count > 0 && count % BLOCK_REPORTING_FREQ as usize == 0 {
                    info!("Found {count} canonical blocks in {:?}", time.elapsed());
                }

                let prev_length_idx = if curr_start_idx > 0 {
                    length_start_indices[curr_start_idx - 1]
                } else {
                    0
                };

                for path in paths[prev_length_idx..curr_length_idx].iter() {
                    if extract_parent_hash_from_path(curr_path).unwrap()
                        == hash_from_path(path).unwrap()
                    {
                        canonical_paths.push(path.file_name().unwrap().to_str().unwrap());
                        curr_path = path;
                        curr_length_idx = prev_length_idx;
                        curr_start_idx -= 1;
                        continue;
                    }
                }
            }

            // final canonical block
            for path in paths[0..curr_length_idx].iter() {
                if extract_parent_hash_from_path(curr_path).unwrap()
                    == hash_from_path(path).unwrap()
                {
                    canonical_paths.push(path.file_name().unwrap().to_str().unwrap());
                    break;
                }
            }

            info!("Canonical chain discovery finished");
            info!(
                "Found {} blocks in the canonical chain in {:?}",
                canonical_paths.len() + 1, // +1 for starting block
                time.elapsed()
            );
            canonical_paths.reverse();

            // add all blocks successive to the canonical chain
            for path in paths[successive_idx..]
                .iter()
                .filter(|p| length_from_path(p).is_some())
            {
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
            "Blocks dir path {blocks_dir:?} does not exist!"
        )))
    }
}
