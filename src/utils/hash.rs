use sha1_smol::Sha1;
use std::{
    fs,
    io::{self, Read},
    path::Path,
};
use thiserror::Error;

/// Computes the hash of a file at `path`.
///
/// # Errors
/// If `path` can't be opened for read.
pub fn hash_file(path: &Path) -> Result<Sha1, HashPathError> {
    let file = fs::read(path).map_err(HashPathError::Io)?;
    let data: Box<[u8]> = file.bytes().flatten().collect();

    Ok(Sha1::from(data))
}

/// Computes the hash of every path in `path`, and then hash those hashes.
/// Applied recursively.
///
/// # Errors
/// If `path` or any of its children can't be opened for read.
pub fn hash_dir(path: &Path) -> Result<Sha1, HashPathError> {
    let mut dir = fs::read_dir(path).map_err(HashPathError::Io)?;
    let mut hashes: Vec<Sha1> = Vec::new();

    while let Some(Ok(file)) = dir.next() {
        let file_type = file.file_type().map_err(HashPathError::Io)?;

        let hash = if file_type.is_file() {
            hash_file(&file.path())
        } else if file_type.is_dir() {
            hash_dir(&file.path())
        } else {
            Err(HashPathError::Symlink)
        }?;

        hashes.push(hash);
    }

    let hexdigests = hashes
        .iter()
        .map(|h| h.digest().to_string())
        .collect::<Box<[String]>>()
        .concat();
    Ok(Sha1::from(hexdigests.as_bytes()))
}

#[derive(Error, Debug)]
pub enum HashPathError {
    #[error("couldn't hash path: {0:?}")]
    Io(io::Error),
    #[error("symlinks are not supported.")]
    Symlink,
}

#[cfg(test)]
mod test {
    use std::{fs, path::Path};

    use super::{hash_dir, hash_file};

    const TEST_PATH: &str = "/tmp/cardinal/test";

    fn setup_path() {
        fs::create_dir_all(TEST_PATH).ok();
    }

    fn cleanup_path() {
        fs::remove_dir_all(TEST_PATH).ok();
    }

    #[test]
    fn expected_file_hash() {
        setup_path();
        let path = Path::new(TEST_PATH).join("hash_file");
        let contents = "nyan".as_bytes();

        fs::write(&path, contents).expect("can't write test file to '{path}'.");

        assert_eq!(
            hash_file(&path)
                .expect("can't hash test file.")
                .digest()
                .to_string(),
            "046385855fc9580393853d8e81f240b66fe9a7b8"
        );

        cleanup_path();
    }

    #[test]
    fn expected_dir_hash() {
        setup_path();
        let path = Path::new(TEST_PATH).join("hash_dir");

        fs::create_dir(&path).expect("can't create test dir.");
        fs::write(path.join("nyan"), "nyan".as_bytes()).expect("can't create test file.");
        fs::write(path.join("meow"), "meow".as_bytes()).expect("can't create test file.");

        assert_eq!(
            hash_dir(&path)
                .expect("can't hash test directory.")
                .digest()
                .to_string(),
            "ed2ace949f083602446f66c68a5f38f1a3495542"
        );

        fs::remove_dir_all(path).ok();
        cleanup_path();
    }
}
