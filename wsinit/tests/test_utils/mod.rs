mod thread_test_path;

use cargo_wsinit::TomlFile;
use std::fs::File;
use std::io::prelude::*;
pub use thread_test_path::ThreadTestPath;

#[allow(dead_code)] // Used by one test crate but not another, causing a warning
pub fn overwrite_file(toml_file: &TomlFile, contents: &str) {
    File::create(&toml_file.as_path())
        .unwrap()
        .write_all(contents.as_bytes())
        .unwrap();
}

pub fn read_file(toml_file: &TomlFile) -> String {
    let mut file_contents = String::new();
    File::open(&toml_file.as_path())
        .unwrap()
        .read_to_string(&mut file_contents)
        .unwrap();
    file_contents
}
