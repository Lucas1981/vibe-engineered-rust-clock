# Analog clock

Build a live analog clock as an isolated project under `./clock` only — ignore sibling Rust folders. Keep the code small and readable; do not add unused APIs “for later.” Put each part of the renderer in its own function (and modules where it helps: canvas, text, face).

1. [x] Project scaffold: create `scaffold.sh` and `build-and-run.sh` recording the setup/`cargo run` commands, with brief comments on terse flags.
2. [x] Canvas: a thin type to set up, present, and tear down a drawable screen (`minifb` window + `tiny-skia` pixmap).
3. [x] Draw a black circle on a white background.
4. [x] Draw numerals 1..12 in clock positions. Render with `fontdue` (TrueType glyphs into the pixmap) — no hand-drawn digit bitmaps or custom stroke fonts.
5. [x] Draw hour / minute / second hands: hour 50% radius × 3px, minute 75% × 2px, second 90% × 1px.
6. [x] Continuous frame loop: each refresh clear → white background → circle → numbers → hands → present (via the dedicated draw functions).
7. [x] Drive hand angles from local wall-clock time (`chrono`).

Stack:

- `tiny-skia` — all 2D paths, strokes, and fills into a pixmap.
- `minifb` — OS window lifecycle and presenting the pixel buffer. Convert pixmap RGBA → `0RGB` on present. No `winit` or `softbuffer`.
- `fontdue` — load/embed a `.ttf`, rasterize glyphs, blend coverage into the pixmap. No other text stacks unless replacing this choice explicitly.
- `chrono` (with `clock`) — local hour, minute, and second.
