#!/usr/bin/env bash

cargo build --release --package holochain-manager-uniffi
cargo run --bin uniffi-bindgen generate --library ../../target/release/libholochain_manager_uniffi.so --language kotlin --out-dir out
cargo run --bin uniffi-bindgen generate --library ../../target/release/libholochain_manager_uniffi.so --language swift --out-dir out