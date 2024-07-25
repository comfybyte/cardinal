use std::{env, ffi::OsString, fs, path::PathBuf};

use crate::utils::xdg;

use super::Store;

const TEST_DIR: &str = "/tmp/cardinal/tests";

#[test]
fn store_works() {
    cleanup();
    setup();

    let store = Store::default();
    assert!(store.create().is_ok());

    setup_dummy_files();
    assert!(store.add(&PathBuf::from(TEST_DIR).join("nyan")).is_ok());
    assert!(store.add(&PathBuf::from(TEST_DIR).join("meow")).is_ok());

    assert!(store
        .delete(&OsString::from(
            "046385855fc9580393853d8e81f240b66fe9a7b8-nyan"
        ))
        .is_ok());
    assert!(store.delete(&OsString::from("woof")).is_err());

    cleanup();
}

fn setup_dummy_files() {
    fs::write(PathBuf::from(TEST_DIR).join("nyan"), "nyan").expect("failed to setup dummy files.");
    fs::write(PathBuf::from(TEST_DIR).join("meow"), "meow").expect("failed to setup dummy files.")
}

fn setup() {
    env::set_var("CARDINAL_DATA", TEST_DIR);
    fs::create_dir_all(TEST_DIR).ok();
}

fn cleanup() {
    fs::remove_dir_all(xdg::store()).ok();
    fs::remove_dir_all(TEST_DIR).ok();
}
