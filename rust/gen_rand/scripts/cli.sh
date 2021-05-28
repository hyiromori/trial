#!/usr/bin/env bash
THIS_DIR="$(cd "$(dirname "${BASH_SOURCE}")"; pwd)"
cd "${THIS_DIR}/../"
cargo build --release
"${THIS_DIR}/../target/release/gen_rand"
