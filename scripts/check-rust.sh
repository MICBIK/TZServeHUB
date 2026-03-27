#!/bin/bash
set -e
cd src-tauri
cargo check
cargo test
echo "Rust compile check passed"
