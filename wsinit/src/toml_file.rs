use std::fmt::{Display, Formatter, Result as FmtResult};
use std::ops::Deref;
use std::path::PathBuf;

#[derive(Clone, Debug)]
pub struct TomlFile(PathBuf);

impl TomlFile {
    pub(crate) fn new(path: PathBuf) -> TomlFile {
        TomlFile(path)
    }
}

impl Deref for TomlFile {
    type Target = PathBuf;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl Display for TomlFile {
    fn fmt(&self, f: &mut Formatter<'_>) -> FmtResult {
        write!(f, "{}", self.to_str().unwrap())
    }
}
