use std::fs;
use std::path::PathBuf;

pub fn read_file_to_string(path: &PathBuf) -> String {
    fs::read_to_string(path).expect("Unable to read file")
}

pub fn write_string_to_file(path: &PathBuf, data: String) {
    fs::write(path, data).expect("Unable to write file");
}
