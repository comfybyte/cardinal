use std::{env, path::PathBuf};

const DIR_NAME: &str = "cardinal";

/// Gets the data directory for Cardinal.
/// Can be overriden if `$CARDINAL_DATA` is set (for testing).
///
/// # Panics
/// If neither `$XDG_DATA_HOME` nor `$HOME` are set.
#[must_use]
pub fn data_dir() -> PathBuf {
    if let Ok(path) = env::var("CARDINAL_DATA") {
        return path.into();
    }

    let mut dir: PathBuf = env::var("XDG_DATA_HOME")
        .unwrap_or_else(|_| {
            let mut home = env::var("HOME").expect("Neither $XDG_DATA_HOME nor $HOME are set.");
            home.push_str("/.local/share");
            home
        })
        .into();
    dir.push(DIR_NAME);
    dir
}

#[must_use]
pub fn store() -> PathBuf {
    data_dir().join("store")
}
