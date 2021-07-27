use std::fs::OpenOptions;
use std::io::prelude::*;
use std::path::Path;

pub fn _open_file(name: String) -> std::fs::File {
    let file = OpenOptions::new()
        .write(true)
        .append(true)
        .open(name.as_str())
        .unwrap();
    file
}

pub fn _create_file(name: String) -> std::fs::File {
    let file = std::fs::OpenOptions::new()
        .write(true)
        .create(true)
        .read(true)
        .open(name.as_str())
        .expect("File could not be created.");
    file
}

pub fn _delete_file(name: String) {
    if Path::new(name.as_str()).exists() {
        std::fs::remove_file(name.as_str())
            .expect("File could not be deleted.");
    }
}

pub fn _write_to_file(file_name: String, entry: &str) {
    let mut file = _open_file(file_name);
    writeln!(file, "{}", entry)
        .expect("File could not be written into.");
}
