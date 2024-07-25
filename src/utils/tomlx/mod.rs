use std::{
    any::{Any, TypeId},
    fs::OpenOptions,
    io::{self, Read},
    path::PathBuf,
};

use toml::{value::Datetime, Table, Value};

/// Reads a TOML file at `path` and parses it into a table.
///
/// # Errors
/// If `path` can't be opened for reading, or isn't valid UTF-8 or TOML table.
pub fn read_toml(path: PathBuf) -> Result<Table, ReadTomlError> {
    let file = OpenOptions::new()
        .read(true)
        .open(path)
        .map_err(ReadTomlError::Io)?;

    let file = String::from_utf8(file.bytes().flatten().collect())
        .map_err(|_| ReadTomlError::ParseUTF8)?;

    file.parse::<Table>().map_err(|_| ReadTomlError::ParseToml)
}

#[derive(Debug)]
pub enum ReadTomlError {
    Io(io::Error),
    ParseUTF8,
    ParseToml,
}

/// Extends `toml::Table`.
pub trait ExtendedTable {
    /// Gets value of `key` if it is a `T`.
    ///
    /// # Errors
    /// If `key` is missing or not a `T`.
    fn get_checked<T: Any + ValidValue + 'static>(&self, key: &str) -> Result<T, CheckError>;
}

impl ExtendedTable for toml::Table {
    fn get_checked<T: Any + ValidValue + 'static>(&self, key: &str) -> Result<T, CheckError> {
        let Some(value) = self.get(key) else {
            return Err(CheckError::Missing);
        };
        let Some(value) = value.downcast_copy::<T>() else {
            return Err(CheckError::Mismatch);
        };

        Ok(value)
    }
}

#[derive(Debug)]
pub enum CheckError {
    Missing,
    Mismatch,
}

pub trait ExtendedValue: Clone {
    /// Gets a copy of the inner value.
    fn downcast_copy<T: Any + ValidValue + 'static>(&self) -> Option<T>;
}

impl ExtendedValue for toml::Value {
    fn downcast_copy<T: Any + ValidValue + 'static>(&self) -> Option<T> {
        let value: Box<dyn Any> = match self.clone() {
            Self::String(v) => Box::new(v),
            Self::Integer(v) => Box::new(v),
            Self::Float(v) => Box::new(v),
            Self::Boolean(v) => Box::new(v),
            Self::Datetime(v) => Box::new(v),
            Self::Array(v) => Box::new(v),
            Self::Table(v) => Box::new(v),
        };

        if (*value).type_id() == TypeId::of::<T>() {
            let Ok(value) = value.downcast::<T>() else {
                unreachable!()
            };

            Some(*value)
        } else {
            None
        }
    }
}

/// Marker for types that can be downcasted from `toml::Value`.
///
/// Used when performing typechecked table access (such as `ExtendedTable::get_checked`).
pub trait ValidValue: Any {}
impl ValidValue for String {}
impl ValidValue for i64 {}
impl ValidValue for f64 {}
impl ValidValue for bool {}
impl ValidValue for Datetime {}
impl ValidValue for Vec<Value> {}
impl ValidValue for Table {}

#[cfg(test)]
#[allow(clippy::unwrap_used)]
mod tests {
    use super::{ExtendedTable, ExtendedValue};

    #[test]
    fn extended_table() {
        let table_str = "
            [files.\".zshrc\"]
            source = \".zshrc\"
            ";
        let dummy_table = table_str
            .parse::<toml::Table>()
            .expect("dummy table in test `extended_table` is malformatted.");

        let files = dummy_table.get_checked::<toml::Table>("files");
        assert!(files.is_ok());

        let dummy_file = files.unwrap().get_checked::<toml::Table>(".zshrc");
        assert!(dummy_file.is_ok());
        assert!(dummy_file.unwrap().get_checked::<String>("source").is_ok());
    }

    #[test]
    fn extended_value() {
        let dummy_value = toml::Value::Integer(42);

        assert!(dummy_value.downcast_copy::<i64>().is_some());
    }
}
