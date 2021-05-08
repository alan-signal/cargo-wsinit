use std::fmt::Debug;
use std::fs;
use std::fs::{File, OpenOptions};
use std::io::prelude::*;
use std::io::{Error as IoError, ErrorKind, SeekFrom};
use std::ops::Deref;
use std::path::{Path, PathBuf};

use walkdir::WalkDir;

use crate::options::FileExistsBehaviour;
use crate::options::Options;
use crate::toml_editor;
use crate::toml_file::TomlFile;

#[derive(Debug)]
pub enum Error {
    FileAlreadyExists,
    GenericCreationError(IoError),
    ReadError(IoError),
    ParseError,
    WriteError(IoError),
}

pub struct Workspace {
    toml: TomlFile,
    options: Options,
}

impl Workspace {
    pub fn new(options: Options) -> Workspace {
        let path = &options.path;
        let toml = TomlFile::new(path.join(Path::new("Cargo.toml")));

        Workspace { toml, options }
    }

    pub fn toml(&self) -> &TomlFile {
        &self.toml
    }

    pub fn update_toml(&self) -> Result<TomlFile, Error> {
        self.create_path()
            .map_err(|err| Error::GenericCreationError(err))?;

        let mut sub_crates = self.find_sub_crates();
        sub_crates.sort();

        let mut file = self.open_file()?;

        self.write_toml_to_file(&mut file, &sub_crates)
            .map(|_| self.toml.clone())
    }

    fn path(&self) -> &PathBuf {
        &self.options.path
    }

    fn create_path(&self) -> Result<(), IoError> {
        fs::create_dir_all(self.path())
    }

    fn find_sub_crates(&self) -> Vec<String> {
        let root = &self.options.path;
        WalkDir::new(root)
            .into_iter()
            .filter_map(|v| v.ok())
            .filter(|d| {
                d.file_name()
                    .to_str()
                    .map(|f| f == "Cargo.toml")
                    .unwrap_or(false)
            })
            .filter_map(|d| {
                d.into_path()
                    .parent()
                    .filter(|p| p != root)
                    .map(|p| p.to_str().map(|a| a.to_string()))
            })
            .filter_map(|z| z)
            .map(|p| {
                if p.starts_with("./") {
                    p[2..].to_string()
                } else {
                    p
                }
            })
            .collect()
    }

    fn open_file(&self) -> Result<File, Error> {
        OpenOptions::new()
            .write(true)
            .read(self.options.existing_file_behaviour == FileExistsBehaviour::Update)
            .create_new(self.options.existing_file_behaviour.create_new())
            .create(self.options.existing_file_behaviour != FileExistsBehaviour::Update)
            .open(self.toml.deref())
            .map_err(|err| match err.kind() {
                ErrorKind::AlreadyExists => Error::FileAlreadyExists,
                _ => Error::GenericCreationError(err),
            })
    }

    fn write_toml_to_file(&self, file: &mut File, sub_crates: &[String]) -> Result<(), Error> {
        let toml_content = match self.options.existing_file_behaviour {
            FileExistsBehaviour::Update => {
                Workspace::read_toml(file).map_err(|err| Error::ReadError(err))?
            }
            _ => "".to_string(),
        };

        let new_file_content = toml_editor::toml_update(toml_content.as_str(), &sub_crates[..])
            .map_err(|_| Error::ParseError)?;

        Workspace::write_toml(file, new_file_content).map_err(|err| Error::WriteError(err))
    }

    fn write_toml(file: &mut File, toml: String) -> Result<(), IoError> {
        file.set_len(0)?;
        file.seek(SeekFrom::Start(0))?;
        file.write_all(toml.as_bytes())
    }

    fn read_toml(file: &mut File) -> Result<String, IoError> {
        let mut toml = String::new();
        file.read_to_string(&mut toml)?;
        Ok(toml)
    }
}
