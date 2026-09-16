#!/usr/bin/env bash
set -euo pipefail

# --check: verify formatting without rewriting source files.
cargo fmt --manifest-path clock/Cargo.toml -- --check

# --release: same profile as ./build-and-run.sh.
cargo clippy --release --manifest-path clock/Cargo.toml

cargo build --release --manifest-path clock/Cargo.toml
