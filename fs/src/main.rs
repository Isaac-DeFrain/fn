use std::{
    ffi::OsStr,
    fs, io,
    path::{Path, PathBuf},
    time::Instant,
    vec::IntoIter, mem::size_of_val,
};

use bytesize::ByteSize;

use glob::{glob, GlobError};

fn main() {
    let time = Instant::now();
    let pattern = env!("BLOCK_FILES_PATTERN");

    // let (num_files, size_msg, iterator) = sort_no_rename(pattern);
    // for path in iterator {
    //     println!("{:?}", path.file_name().unwrap());
    // }

    let num_files = glob_unpad_dir(pattern).unwrap();
    let total = time.elapsed();

    println!("\n~~~ Stats ~~~");
    // println!("{size_msg}");
    println!("Time elapsed: {total:?}");
    println!("Num files:    {num_files:?}\n");
}

#[allow(dead_code)]
fn get_blockchain_length(file_name: &OsStr) -> Option<u32> {
    file_name
        .to_str()?
        .split('-')
        .fold(None, |acc, x| match x.parse::<u32>() {
            Err(_) => acc,
            Ok(x) => Some(x),
        })
}

#[allow(dead_code)]
fn sort_no_rename<'a>(pattern: &str) -> (usize, String, IntoIter<PathBuf>) {
    let mut glob_vec: Vec<PathBuf> = glob(pattern)
        .expect("Failed to read glob pattern")
        .filter_map(|x| x.ok())
        .collect();
    glob_vec.sort_by(|x, y| {
        get_blockchain_length(x.as_os_str()).cmp(&get_blockchain_length(y.as_os_str()))
    });

    let size_msg = format!("Size of vec:  {}", ByteSize::b(size_of_val(&*glob_vec) as u64)); 
    (glob_vec.len(), size_msg, glob_vec.into_iter())
}

#[allow(dead_code)]
fn glob_rename_dir(pattern: &str) -> Result<usize, GlobError> {
    let mut num = 0;
    for entry in glob(pattern).expect("Failed to read glob pattern") {
        match entry {
            Ok(path) => {
                let new_path = pad_height(path.clone());
                fs::rename(path, &new_path).unwrap();
                num += 1;
            }
            Err(err) => println!("{err:?}"),
        }
    }
    Ok(num)
}

#[allow(dead_code)]
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
                    acc.push('-');
                } else {
                    acc.push('-');
                    acc.push_str(part);
                    acc.push('-');
                }
            } else {
                acc.push_str(part);
            }
            acc
        });

    result.set_file_name(&new_fname);
    result
}

#[allow(dead_code)]
fn glob_unpad_dir(pattern: &str) -> Result<usize, GlobError> {
    let mut num = 0;
    for entry in glob(pattern).expect("Failed to read glob pattern") {
        match entry {
            Ok(path) => {
                let new_path = unpad_height(path.clone());
                fs::rename(path, &new_path).unwrap();
                num += 1;
            }
            Err(err) => println!("{err:?}"),
        }
    }
    Ok(num)
}

#[allow(dead_code)]
fn unpad_height(path: impl AsRef<Path>) -> PathBuf {
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
                let height = height.trim_start_matches('0');
                acc.push('-');
                acc.push_str(height);
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
