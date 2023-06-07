use std::{
    ffi::OsStr,
    fs, io,
    path::{Path, PathBuf},
};

use glob::{glob, GlobError};

fn main() {
    let pattern = env!("BLOCK_FILES_PATTERN");
    for path in sort_no_rename(pattern) {
        println!("{:?}", path.file_name().unwrap());
    }
}

fn get_blockchain_length(file_name: &OsStr) -> Option<u32> {
    file_name
        .to_str()?
        .split('-')
        .fold(None, |acc, x| match x.parse::<u32>() {
            Err(_) => acc,
            Ok(x) => Some(x),
        })
}

fn sort_no_rename<'a>(pattern: &str) -> Vec<PathBuf> {
    let mut glob_vec: Vec<PathBuf> = glob(pattern)
        .expect("Failed to read glob pattern")
        .filter_map(|x| x.ok())
        .collect();
    glob_vec.sort_by(|x, y| {
        get_blockchain_length(x.as_os_str()).cmp(&get_blockchain_length(y.as_os_str()))
    });
    glob_vec
}

#[allow(dead_code)]
fn glob_rename_dir(pattern: &str) -> Result<(), GlobError> {
    for entry in glob(pattern).expect("Failed to read glob pattern") {
        match entry {
            Ok(path) => {
                let new_path = pad_height(path.clone());
                fs::rename(path, &new_path).unwrap();
            }
            Err(err) => println!("{err:?}"),
        }
    }
    Ok(())
}

fn pad_height(path: impl AsRef<Path>) -> PathBuf {
    let path = path.as_ref();
    let mut result = path.to_owned();
    let fname = path.file_name().unwrap().to_owned().into_string().unwrap();

    let new_fname = fname
        .split('-')
        .enumerate()
        .fold(String::new(), |acc, (n, part)| {
            let mut acc = acc;
            if n == 1 {
                let height = part.to_string();
                if height.len() < 6 {
                    let mut padded = "0".repeat(6 - height.len());
                    padded.push_str(&height);

                    acc.push('-');
                    acc.push_str(&padded);
                } else {
                    acc.push('-');
                    acc.push_str(part);
                }
            } else {
                if n != 0 {
                    acc.push('-');
                }
                acc.push_str(part);
            }
            acc
        });

    result.set_file_name(&new_fname);
    result
}

#[allow(dead_code)]
fn read_curr_dir() -> io::Result<()> {
    let mut entries = fs::read_dir(".")?
        .map(|res| res.map(|e| e.path()))
        .collect::<Result<Vec<_>, io::Error>>()?;

    // The order in which `read_dir` returns entries is not guaranteed. If reproducible
    // ordering is required the entries should be explicitly sorted.

    entries.sort();
    for (n, entry) in entries.iter().enumerate() {
        println!("{n}: {entry:?}");
    }

    Ok(())
}
