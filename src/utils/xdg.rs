use std::{env, path::PathBuf};

const DIR_NAME: &str = "cardinal";

/// Gets the data directory for Cardinal.
///
/// # Panics
/// If neither `$XDG_DATA_HOME` nor `$HOME` are set.
#[must_use]
pub fn data_dir() -> PathBuf {
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
