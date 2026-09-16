#!/usr/bin/env bash
set -euo pipefail

# --release: optimised build — skia path rendering is noticeably faster than debug.
# -p clock: target only the `clock` package inside the workspace (or the single crate).
cargo run --release --manifest-path clock/Cargo.toml
