use std::{
    fs::{create_dir_all, File},
    path::PathBuf,
};

/// If path exists, assert that it's a file,
/// else create the path as a file
pub fn check_file(file: &PathBuf) {
    if file.exists() {
        assert!(file.is_file(), "{} must be a file!", file.display());
    } else {
        create_dir_all(file.parent().unwrap()).unwrap();
        File::create(file).unwrap();
    }
}

/// If path exists, assert that it's a directory,
/// else create the path as a directory
pub fn check_dir(dir: &PathBuf) {
    if dir.exists() {
        assert!(dir.is_dir(), "{} must be a dir!", dir.display());
    } else {
        create_dir_all(dir).unwrap()
    }
}
