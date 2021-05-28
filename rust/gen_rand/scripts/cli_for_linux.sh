#!/usr/bin/env bash
THIS_DIR="$(cd "$(dirname "${BASH_SOURCE}")"; pwd)"
cd "${THIS_DIR}/../"

# https://stackoverflow.com/questions/41761485/how-to-cross-compile-from-mac-to-linux
rustup target add x86_64-unknown-linux-gnu
cargo build --release --target=x86_64-unknown-linux-gnu
