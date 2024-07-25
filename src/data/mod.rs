use std::path::PathBuf;

use file::{FileItem, FromTableError};
use thiserror::Error;

use crate::utils::tomlx::{self, ExtendedTable, ExtendedValue};

const CARDINAL_CONFIG_NAME: &str = "cardinal.toml";

pub mod file;
pub mod store;

/// Holds the runtime data.
#[derive(Debug)]
pub struct Cardinal {
    pub files: Vec<FileItem>,
}

impl Cardinal {
    /// Instantiate from a `cardinal.toml` file.
    ///
    /// # Errors
    /// If there is no `cardinal.toml` file in the current working directory,
    /// or its schema is invalid.
    pub fn new() -> Result<Self, CardinalError> {
        let config =
            tomlx::read_toml(PathBuf::from(CARDINAL_CONFIG_NAME)).map_err(CardinalError::Read)?;

        let files_table = config
            .get_checked::<toml::Table>("files")
            .map_err(|err| CardinalError::InvalidField("files".to_string(), err))?;
        let mut files = Vec::<FileItem>::with_capacity(files_table.len());

        for (file_path, file_data) in files_table {
            let Some(file_data) = file_data.downcast_copy::<toml::Table>() else {
                return Err(CardinalError::InvalidFileType(file_path));
            };
            let file = FileItem::from_table(&file_path, &file_data)
                .map_err(CardinalError::InvalidFileSchema)?;
            files.push(file);
        }

        Ok(Self { files })
    }
}

#[derive(Error, Debug)]
pub enum CardinalError {
    #[error("Failed to read configuration file: {0:?}")]
    Read(tomlx::ReadTomlError),
    #[error("Read configuration file but a field '{0}' is not valid: {1:?}.")]
    InvalidField(String, tomlx::CheckError),
    #[error("'files.\"{0}\"' must be a table.")]
    InvalidFileType(String),
    #[error("Failed to parse 'files.\"{0:?}\"'")]
    InvalidFileSchema(FromTableError),
}
