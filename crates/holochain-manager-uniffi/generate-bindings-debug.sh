#!/usr/bin/env bash

cargo build --package holochain-manager-uniffi && \
(
  cargo run --bin uniffi-bindgen generate --library ../../target/debug/libholochain_manager_uniffi.so --language kotlin --out-dir out
  cargo run --bin uniffi-bindgen generate --library ../../target/debug/libholochain_manager_uniffi.so --language swift --out-dir out
)