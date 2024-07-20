_default:
  just --list

watch:
  bacon clippy

build:
  cargo build --release
