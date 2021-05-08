use std::path::{Path, PathBuf};

pub struct Options {
    pub(crate) path: PathBuf,
    pub(crate) existing_file_behaviour: FileExistsBehaviour,
}

#[derive(PartialEq)]
pub enum FileExistsBehaviour {
    /// If the toml file already exists, the tool will stop.
    Halt,

    /// If the toml file already exists, it will be modified.
    Update,

    /// If the toml file already exists, it will be overwritten.
    Overwrite,
}

impl Options {
    /// Create a new options struct for the specified path (not including the Cargo.toml file itself)
    /// and the specified existing file behaviour.
    pub fn new(path: &str, overwrite: FileExistsBehaviour) -> Options {
        Options {
            path: Path::new(path).into(),
            existing_file_behaviour: overwrite,
        }
    }
}

impl FileExistsBehaviour {
    pub(crate) fn create_new(&self) -> bool {
        match self {
            FileExistsBehaviour::Halt => true,
            FileExistsBehaviour::Update => false,
            FileExistsBehaviour::Overwrite => false,
        }
    }
}
