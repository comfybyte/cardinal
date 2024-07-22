use std::path::PathBuf;

use thiserror::Error;

use crate::utils::tomlx::{self, ExtendedTable};

/// Represents an item under `files` in the configuration file.
#[derive(Debug)]
pub struct FileItem {
    /// Where to link the `source` to.
    pub path: PathBuf,
    /// Which file to link to `path`.
    pub source: PathBuf,
}

impl FileItem {
    #[must_use]
    pub const fn new(path: PathBuf, source: PathBuf) -> Self {
        Self { path, source }
    }

    /// Creates a new instance from a `Table`.
    ///
    /// # Errors
    /// If any required field is missing or has an invalid type.
    pub fn from_table(k: &str, v: &toml::Table) -> Result<Self, FromTableError> {
        let source = v.get_checked::<String>("source").map_err(|err| {
            FromTableError::InvalidField(k.to_string(), "source".to_string(), err)
        })?;
        let source = PathBuf::from(source);
        let path = PathBuf::from(k);

        Ok(Self::new(path, source))
    }
}

#[derive(Error, Debug)]
pub enum FromTableError {
    #[error("File '{0}' has an invalid field '{1}': {2:?}")]
    InvalidField(String, String, tomlx::CheckError),
}
