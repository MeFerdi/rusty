use std::fs::{OpenOptions};
use std::io::{Write};
use std::path::Path;

pub fn open_or_create<P: AsRef<Path>>(path: &P, content: &str) {
    let file_result = OpenOptions::new()
        .create(true)
        .append(true)
        .open(path);

    match file_result {
        Ok(mut file) => {
            if let Err(e) = write!(file, "{}", content) {
                panic!("Failed to write to file: {}", e);
            }
        }
        Err(e) => panic!("Failed to open or create file: {}", e),
    }
}