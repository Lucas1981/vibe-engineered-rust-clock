# Codebase Review — rust-clock

Review date: 2026-09-16  
Scope: entire repository (`clock/` crate, scripts, docs)

This is a small, well-scoped analog clock. The module split (`canvas`, `face`, `text`, `hands`) matches `AGENTS.md`, the draw pipeline is easy to follow, and the code reads like intentional learning material rather than accidental complexity. The suggestions below are ordered roughly by impact.

---

## High impact

### 1. Reuse the present buffer instead of allocating every frame

`Canvas::present` builds a fresh `Vec<u32>` on every call:

```35:42:clock/src/canvas.rs
        let buffer: Vec<u32> = self
            .pixmap
            .data()
            .chunks_exact(4)
            // tiny-skia stores pixels as premultiplied RGBA; for fully-opaque
            // shapes the channel values equal their straight-alpha equivalents.
            .map(|p| ((p[0] as u32) << 16) | ((p[1] as u32) << 8) | p[2] as u32)
            .collect();
```

At 600×600 that is ~1.4 MB allocated and freed 60+ times per second. `LEARNINGS.md` already documents a reusable `present_buf` pattern, but the live code no longer uses it. Store a `Vec<u32>` on `Canvas`, resize it once in `new`, and fill it in place (e.g. `iter_mut().zip(pixels.as_chunks::<4>().0)`). This is the single largest runtime win available without changing the rendering model.

### 2. Cache numeral glyph rasterisation

`draw_numerals` calls `font.rasterize` for every digit on every frame. Digits 1–12 never change. Pre-rasterise them once at startup (e.g. `[(String, Metrics, Vec<u8>); 12]` or a small struct keyed by digit) and reuse the bitmaps in the draw loop. For a clock that runs indefinitely, this removes redundant CPU work every frame.

### 3. `LEARNINGS.md` is out of sync with the code

Two sections describe patterns that no longer exist:

- **Anonymous `{ ... }` scopes in `main`** — current `main.rs` holds `pixmap_mut()` across the whole loop body without a nested block. That works because `present()` is called after the borrow ends each iteration; the learning note about needing a block is misleading for readers following the repo today.
- **`present_buf` + `iter_mut().zip(...)`** — documented in LEARNINGS but not implemented in `canvas.rs`.

Either update LEARNINGS to match the current code, or implement the documented patterns and describe why. Stale docs are worse than no docs for a learning repo.

---

## Medium impact

### 4. Centralise shared geometry and colour constants

Layout values are spread across files with no single source of truth:

| Constant | Location | Value |
|----------|----------|-------|
| Window size | `main.rs` | 600×600 |
| Face radius | `main.rs` | 270 |
| Stroke width | `face.rs` | 4 |
| Numeral radius factor | `text.rs` | 0.78 |
| Font size | `text.rs` | 30 px |
| Hand length factors | `hands.rs` | 0.50 / 0.75 / 0.90 |

Consider a small `layout.rs` or a `const` block in `main.rs` passed into draw functions, so margin (30 px), numeral inset, and hand proportions stay consistent when resizing the window.

### 5. Use `tiny_skia::Color` consistently

`face.rs` uses `Color::WHITE` for the background but `set_color_rgba8(0, 0, 0, 255)` for the stroke. `hands.rs` repeats the raw RGBA tuple. Prefer `Color::BLACK` (or one shared `const HAND_COLOR: Color`) everywhere for readability and to avoid magic byte literals.

### 6. Redundant cast in font loading

```18:18:clock/src/main.rs
    let font = Font::from_bytes(font_bytes as &[u8], FontSettings::default())
```

`include_bytes!` already yields `&'static [u8]`. The `as &[u8]` cast adds noise without changing behaviour.

### 7. Parameter naming: `pix` vs `pixmap`

Draw functions take `pixmap: &mut Pixmap`; `main` binds `let pix = canvas.pixmap_mut()`. Pick one name (`pixmap` matches the type and module style) for consistency.

### 8. Frame pacing / CPU usage

The main loop spins as fast as possible with no sleep or vsync. On battery-powered machines this will peg a core. A simple improvement: sleep until the next whole second (or next frame budget) after `present()`, since sub-second hand motion already uses nanoseconds. Alternatively, document that busy-looping is intentional for maximum smoothness.

### 9. Font asset and licensing

`clock/assets/arial-bold.ttf` (~750 KB) is committed and embedded at compile time. Arial is a proprietary Microsoft font; redistributing it may require a license depending on how the project is shared. For an open or portable repo, consider:

- A clearly licensed alternative (e.g. Liberation Sans, DejaVu Sans, Inter OFL).
- A note in README about font provenance and license.

Also update `scaffold.sh` to mention creating `assets/` and placing a font — today it adds dependencies but not the asset the binary requires.

---

## Low impact / polish

### 10. Apply `cargo fmt`

`cargo fmt --check` reports minor diffs in `hands.rs` (import order) and `text.rs` (alignment). Run `cargo fmt` in CI or before commits.

### 11. Address the Clippy hint in `present`

Clippy suggests `as_chunks::<4>()` over `chunks_exact(4)` when converting RGBA to packed pixels. Minor, but aligns with current Rust idioms.

### 12. Consider `Pixmap::fill_rect` for the white background

`face.rs` calls `pixmap.fill(Color::WHITE)`, which fills the entire pixmap. That is correct; if you ever draw only inside the circle, a full fill is still the right clear. No change required unless you split “clear” from “draw face” per `AGENTS.md` wording — today `draw_face` does both, which is fine for this size.

### 13. Error handling strategy

Heavy use of `expect` is reasonable for a toy binary (window open, pixmap alloc, path build). If you want to grow this into a library or CLI tool, propagate `Result` from `Canvas::new` and `present` instead. For the current scope, adding one-line comments on *why* each `expect` is unreachable in normal use would help readers (LEARNINGS already covers the general pattern).

### 14. Root-level workspace layout

The repo root holds scripts and docs; the crate lives under `clock/`. `build-and-run.sh` correctly uses `--manifest-path clock/Cargo.toml`. Optional: add a root `Cargo.toml` workspace with `members = ["clock"]` so `cargo run -p clock` works from the repo root without the script. Not necessary if scripts are the intended entry point.

### 15. `.gitignore`

`target/` at repo root is ignored; `clock/target/` is also matched. Good. If you add IDE or OS cruft at root level, consider `.DS_Store` (already listed) and editor folders as needed.

### 16. Dependency versions

Pinned versions in `Cargo.toml` are fine. No obvious unused dependencies. All four crates match `AGENTS.md`. Periodically `cargo update` and verify — no action required now.

---

## Consistency with `AGENTS.md`

| Requirement | Status |
|-------------|--------|
| Isolated under `./clock` | ✓ |
| `minifb` + `tiny-skia` + `fontdue` + `chrono` | ✓ |
| Separate draw functions / modules | ✓ |
| Black circle, white background | ✓ |
| Numerals 1–12 via fontdue | ✓ |
| Hand sizes 50%/75%/90%, 3/2/1 px | ✓ |
| Continuous loop: clear → face → numbers → hands → present | ✓ (clear is inside `draw_face`) |
| Wall-clock time with smooth motion | ✓ |
| `scaffold.sh` / `build-and-run.sh` | ✓ (scaffold omits font asset step) |

---

## What is already in good shape

- **Module boundaries** — Each rendering concern has one job; `main` is a thin orchestrator.
- **Hand angle math** — Fractional hour/minute/second with nanoseconds gives smooth motion; 12-o’clock at top via `FRAC_PI_2` is consistent in `hands.rs` and `text.rs`.
- **Text blending** — Manual premultiplied alpha in `blend_black_glyph` is correct and well commented; works on any background without a second pass.
- **Borrow structure in `main`** — Mutable borrow of the pixmap ends before `present()` each iteration; no need for extra scopes with the current structure.
- **Scripts** — `set -euo pipefail`, comments on flags, and `--release` in the run script show deliberate setup.
- **Scope discipline** — No unused “for later” APIs; the codebase stays small (~200 lines of Rust).

---

## Suggested priority order

If you tackle these incrementally:

1. Reuse present buffer (performance, matches LEARNINGS).
2. Sync or fix LEARNINGS.md (documentation trust).
3. Cache numeral glyphs (performance).
4. Centralise constants + `Color` usage (maintainability).
5. Frame sleep or vsync note (resource use).
6. Font license / scaffold asset step (distribution hygiene).
7. `cargo fmt` + minor Clippy/cast cleanups (polish).

None of these are blockers for a personal learning project; the code is coherent, readable, and appropriate for its stated purpose.
