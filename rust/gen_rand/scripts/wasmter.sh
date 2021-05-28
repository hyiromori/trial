#!/usr/bin/env bash
THIS_DIR="$(cd "$(dirname "${BASH_SOURCE}")"; pwd)"
cd "${THIS_DIR}/../"

# https://zenn.dev/hyiromori/scraps/9b755345a8fe48
rustup target add wasm32-wasi
cargo build --release --target wasm32-wasi
wasmer run target/wasm32-wasi/release/gen_rand.wasm
