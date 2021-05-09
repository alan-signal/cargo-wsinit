use std::fmt::Debug;
use std::fs::{self, File, OpenOptions};
use std::io::prelude::*;
use std::io::{Error as IoError, ErrorKind, SeekFrom};
use std::ops::Deref;
use std::path::{Path, PathBuf};

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

        let mut sub_crates = self
            .find_sub_crates()
            .map_err(|err| Error::GenericCreationError(err))?;

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

    fn find_sub_crates(&self) -> Result<Vec<String>, IoError> {
        let root = &self.options.path;
        let sub_toml_files = Workspace::search_for_cargo_files(root, 0)?;

        Ok(sub_toml_files
            .iter()
            .map(|p| {
                p.parent()
                    .unwrap()
                    .strip_prefix(root)
                    .unwrap()
                    .to_str()
                    .unwrap()
                    .to_string()
            })
            .collect())
    }

    fn search_for_cargo_files(dir: &PathBuf, depth: i32) -> Result<Vec<PathBuf>, IoError> {
        let mut results: Vec<PathBuf> = vec![];

        // do not look in the workspace root
        if depth > 0 {
            for path in fs::read_dir(dir)? {
                let path = path.unwrap().path();

                if path.is_file() {
                    match path.file_name() {
                        Some(p) if p == "Cargo.toml" => {
                            results.push(path);
                            break;
                        }
                        _ => (),
                    }
                }
            }
        }

        // do not look in sub directories after found a Cargo.toml
        if results.is_empty() {
            for path in fs::read_dir(dir)? {
                let path = path.unwrap().path();

                if path.is_dir() {
                    match path.file_name() {
                        Some(p) if p != "target" => {
                            results.extend(Workspace::search_for_cargo_files(&path, depth + 1)?);
                        }
                        _ => (),
                    }
                }
            }
        }

        Ok(results)
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
        file.set_len(0)?; // in lieu of OpenOptions::truncate which would prevent reading
        file.seek(SeekFrom::Start(0))?;
        file.write_all(toml.as_bytes())
    }

    fn read_toml(file: &mut File) -> Result<String, IoError> {
        let mut toml = String::new();
        file.read_to_string(&mut toml)?;
        Ok(toml)
    }
}
