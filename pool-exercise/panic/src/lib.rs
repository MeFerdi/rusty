use std::fs::File;

pub fn open_file(filename: &str) -> File {
    File::open(filename).unwrap()
}