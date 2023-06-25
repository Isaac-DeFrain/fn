use std::{
    fs::{create_dir_all, File},
    path::PathBuf,
};

pub fn check_file(file: &PathBuf) {
    if file.exists() {
        assert!(file.is_file(), "{} must be a file!", file.display());
    } else {
        create_dir_all(file.parent().unwrap()).unwrap();
        File::create(file).unwrap();
    }
}

pub fn check_dir(dir: &PathBuf) {
    if dir.exists() {
        assert!(dir.is_dir(), "{} must be a dir!", dir.display());
    } else {
        create_dir_all(dir).unwrap()
    }
}
