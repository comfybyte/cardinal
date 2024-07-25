use std::{
    ffi::OsStr,
    fs, io,
    path::{Path, PathBuf},
};

use thiserror::Error;
use tracing::info;

use crate::utils::{hashing, xdg};

#[cfg(test)]
pub mod tests;

pub struct Store {
    /// The file system path to the store.
    pub path: PathBuf,
}

/// A collection of functions to deal with the store.
impl Store {
    /// Initialises an empty store if it doesn't exist, doing nothing if it already exists.
    ///
    /// # Errors
    /// If the store can't be created.
    pub fn create(&self) -> Result<(), CreateError> {
        if let Err(err) = fs::create_dir(&self.path) {
            if err.kind() == io::ErrorKind::AlreadyExists {
                Ok(())
            } else {
                Err(CreateError::Io(err))
            }
        } else {
            info!("initialised empty store.");
            Ok(())
        }
    }

    /// Lists all store paths.
    ///
    /// # Errors
    /// If the store contents can't be read.
    pub fn list(&self) -> Result<Vec<PathBuf>, ListError> {
        Ok(fs::read_dir(&self.path)
            .map_err(ListError::Io)?
            .flatten() // this is very naive and should be changed soon.
            .map(|file| file.path()) // this should also merge with ^ into a flat_map.
            .collect())
    }

    /// Adds a copy of `path` to the store and return the path to that copy.
    ///
    /// # Errors
    /// If the path can't be hashed or copied to the store.
    pub fn add(&self, path: &Path) -> Result<Box<Path>, AddError> {
        let store_path = self.make_store_path_for(path).map_err(AddError::Hashing)?;
        fs::copy(path, &store_path).map_err(AddError::Copy)?;
        Ok(store_path)
    }

    /// Deletes a store `path`. It must be relative to the store itself.
    ///
    /// # Errors
    /// If `path` can't be deleted.
    pub fn delete(&self, name: &OsStr) -> Result<(), DeleteError> {
        let path = xdg::store().join(name);

        if let Err(err) = fs::remove_file(&path) {
            Err(DeleteError::Io(path, err))
        } else {
            Ok(())
        }
    }

    /// Computes a hash for `path` and returns a path to place it in the store.
    ///
    /// # Errors
    /// See [`hashing::hash_path`].
    ///
    /// # Panics
    /// If given a blank path, or one that can't be parsed into UTF-8.
    fn make_store_path_for(&self, path: &Path) -> Result<Box<Path>, hashing::HashPathError> {
        let hash = hashing::hash_path(path)?;

        let filename = path
            .file_name()
            .unwrap_or_else(|| unreachable!())
            .to_str()
            .expect("can't parse file name to UTF-8 string.");

        let mut store_name = hash.digest().to_string();
        store_name.push('-');
        store_name.push_str(filename);
        Ok(self.path.join(store_name).into_boxed_path())
    }
}

impl Default for Store {
    fn default() -> Self {
        Self { path: xdg::store() }
    }
}

// TODO: merge those error enums.
#[derive(Error, Debug)]
pub enum CreateError {
    #[error("IO error when creating the store: {0:?}")]
    Io(io::Error),
}

#[derive(Error, Debug)]
pub enum ListError {
    #[error("IO error when listing store contents: {0:?}")]
    Io(io::Error),
}

#[derive(Error, Debug)]
pub enum AddError {
    #[error("source does not exist.")]
    NotFound,
    #[error("IO error when copying path to the store: '{0:?}'")]
    Copy(io::Error),
    #[error("a copy of the source path already exists.")]
    Conflict,
    #[error("could not compute hash for path: {0}.")]
    Hashing(hashing::HashPathError),
}

#[derive(Error, Debug)]
pub enum DeleteError {
    #[error("IO error when deleting '{0}': {1:?}")]
    Io(PathBuf, io::Error),
}
