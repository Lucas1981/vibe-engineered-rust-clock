#!/usr/bin/env bash
set -euo pipefail

# Create the project directory and initialise a new binary crate.
# --name sets the crate name explicitly (otherwise it defaults to the dir name).
cargo new --name clock clock

cd clock

# Add runtime dependencies with their required feature flags.
# tiny-skia: 2D path/stroke/fill renderer into a raw pixmap.
cargo add tiny-skia

# minifb: lightweight OS window; we only need the default feature set.
cargo add minifb

# fontdue: TrueType rasteriser — no extra features needed for glyph rendering.
cargo add fontdue

# chrono: wall-clock time; `clock` feature enables Local::now().
cargo add chrono --features clock

echo ""
echo "Scaffold complete — run ./build-and-run.sh to compile and launch."
