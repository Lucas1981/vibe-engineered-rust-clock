# Learnings

Notes from building the clock app and unpacking the Rust idioms it uses.

## Anonymous `{ ... }` scopes (borrow checker)

In `main`, drawing is wrapped in a bare block:

```rust
{
    let pixmap = canvas.pixmap_mut();
    // ... draw ...
}

while canvas.is_open() {
    canvas.present();
}
```

Those braces are an **anonymous scope**, not a missing label or heading.

`canvas.pixmap_mut()` borrows `canvas` mutably. That borrow lasts as long as `pixmap` is in scope. Without the extra block, `pixmap` would still be alive when the `while` loop calls `is_open()` / `present()`, and Rust would reject it — you cannot use `canvas` again while a mutable borrow of it exists.

The block ends the borrow so `canvas` is free for the present loop. A helper like `draw_face(&mut canvas)` would achieve the same thing more explicitly.

## `iter_mut().zip(pixels.chunks_exact(4))`

In `Canvas::present`, we convert tiny-skia’s RGBA bytes into minifb’s `0RGB` `u32` buffer:

```rust
for (dst, rgba) in self.present_buf.iter_mut().zip(pixels.chunks_exact(4)) {
    let r = rgba[0] as u32;
    let g = rgba[1] as u32;
    let b = rgba[2] as u32;
    *dst = (r << 16) | (g << 8) | b;
}
```

What each piece does:

| Piece | Role |
|--------|------|
| `present_buf` | `Vec<u32>` of size `width * height` — one packed pixel per slot for minifb |
| `.iter_mut()` | Yields `&mut u32` for each slot so the loop can write `*dst = ...` |
| `pixels` (`pixmap.data()`) | Flat `&[u8]` in RGBA order: `[R,G,B,A, R,G,B,A, ...]` |
| `.chunks_exact(4)` | Groups that byte slice into consecutive 4-byte pixels; shorter leftovers are dropped |
| `.zip(...)` | Walks both iterators in lockstep as `(dst, rgba)` pairs; stops when either side ends |

Alpha (`rgba[3]`) is ignored because this minifb encoding does not use it.

## `*dst` and `&mut` vs C/C++ pointers

In the loop, `dst` is a **mutable reference** (`&mut u32`), not a raw pointer. `*dst = ...` writes through that reference into the actual `u32` — same mental model as `*p = ...` in C.

Rough analogues:

```c
uint32_t *dst = &present_buf[i];
*dst = (r << 16) | (g << 8) | b;
```

```cpp
uint32_t &dst = present_buf[i];
dst = (r << 16) | (g << 8) | b;  // no * needed for a C++ reference
```

| Rust | C | C++ |
|------|---|-----|
| `&T` | roughly `const T*` (non-null, valid) | roughly `const T&` |
| `&mut T` | roughly `T*` with exclusive access | roughly `T&` with exclusive access |
| `*r` / `*r = v` | `*p` / `*p = v` | `*p` or assign through `T&` |
| `*mut T` (raw) | `T*` | `T*` |

Important differences from C/C++:

1. **Validity** — Safe Rust references are never null and must point at a live `T`.
2. **Exclusivity** — While an `&mut T` exists, nothing else may access that data (why the draw scope was needed).
3. **Shared borrows** — `&T` allows many readers and no writers, enforced by the borrow checker.
4. **Rebinding vs writing** — If `dst` is `mut`, `dst = &mut other` reseats the reference; `*dst = v` writes the referent (more like a C pointer binding than a C++ reference, which usually cannot be reseated).
5. **Raw pointers** — `*const T` / `*mut T` are the C-like pointers; dereferencing them requires `unsafe`.

## Why `expect()` appears in product code

`expect` is not test-only. Fallible APIs return `Option` / `Result`; `expect(msg)` means “unwrap or panic with this message” (like `unwrap()`, but with a reason).

Typical choices in Rust:

1. Propagate with `?`
2. Recover (`match` / `if let` / default)
3. Treat as fatal: `unwrap` / `expect` / `panic!`

In this small app, several failures are treated as fatal invariants (bad dimensions, invalid circle inputs, present failure) — closer to “assert we cannot continue” than to a unit-test assertion.

- **In a test:** panic → fail the test  
- **In the app:** panic → crash the process (or thread)

Prefer `expect("why this must succeed")` for programmer bugs or unrecoverable environment problems; prefer `?` / `match` when failure is a normal case the program should handle. Libraries and long-running services usually return `Result` instead of panicking so callers can decide.

## `const` vs `let`, and what `mut` actually means

In `main` we have both:

```rust
const WIDTH: u32 = 600;
const RADIUS: f32 = 270.0;

let font = Font::from_bytes(...).expect("...");
let mut canvas = Canvas::new(WIDTH, HEIGHT, "Clock");
```

### `const` is not “immutable `let`”

`const` declares a **compile-time constant**: the value must be known (or computable) when the program is built. It is not a local stack binding created when `main` runs.

`let` declares a **runtime local**. Even if you never reassign it, it is still created when that scope executes. `font` and `canvas` cannot be `const` because they come from runtime calls (`Font::from_bytes`, `Canvas::new`).

So: not mutable ≠ should be `const`. Many immutable values are still `let`.

### Why `let mut canvas` when we never write `canvas = ...`

`mut` means the binding allows **mutation of the value**, not only reassignment of the name.

We never replace `canvas` with a new `Canvas`, but we do mutate what it owns every frame:

```rust
let pix = canvas.pixmap_mut();  // &mut self
canvas.present();               // &mut self
```

Those methods take `&mut self`, so they need a mutable binding. Without `mut`, the compiler rejects the calls (“cannot borrow as mutable”).

By contrast, `is_open` takes `&self` and would work on a non-`mut` binding alone.

### Mental model

| What you want | Use |
|---|---|
| Fixed compile-time number / config | `const` |
| Runtime value, never change it | `let` |
| Runtime value, change fields / call `&mut self` | `let mut` |
| Runtime value, replace the whole binding (`x = ...`) | also `let mut` |

`canvas` needs `mut` because its **contents** change every frame, even though the variable always refers to the same `Canvas`.
