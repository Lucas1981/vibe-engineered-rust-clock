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

# Numerals font: Liberation Sans Bold (SIL Open Font License 1.1).
# Metrically compatible with Arial; safe to embed and redistribute.
LIBERATION_TTF_URL="https://github.com/liberationfonts/liberation-fonts/files/7261482/liberation-fonts-ttf-2.1.5.tar.gz"
TMP=$(mktemp -d)
curl -fsSL "$LIBERATION_TTF_URL" -o "$TMP/liberation.tar.gz"
tar -xzf "$TMP/liberation.tar.gz" -C "$TMP"
mkdir -p assets
cp "$TMP/liberation-fonts-ttf-2.1.5/LiberationSans-Bold.ttf" assets/liberation-sans-bold.ttf
cp "$TMP/liberation-fonts-ttf-2.1.5/LICENSE" assets/FONT-LICENSE.txt
rm -rf "$TMP"

echo ""
echo "Scaffold complete — run ./build-and-run.sh to compile and launch."
