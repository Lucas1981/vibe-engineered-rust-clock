use fontdue::{Font, Metrics};
use std::f32::consts::{FRAC_PI_2, TAU};
use tiny_skia::Pixmap;

/// Font size used for all clock numerals (px at 96 dpi).
const FONT_PX: f32 = 30.0;

/// Place numerals 1–12 around the clock face at 78 % of `radius`.
pub fn draw_numerals(pixmap: &mut Pixmap, font: &Font, cx: f32, cy: f32, radius: f32) {
    let num_radius = radius * 0.78;

    for n in 1u32..=12 {
        // 12 sits at the top (−π/2); each step is 1/12 of a full turn.
        let angle = (n as f32 / 12.0) * TAU - FRAC_PI_2;
        let x = cx + angle.cos() * num_radius;
        let y = cy + angle.sin() * num_radius;

        draw_str(pixmap, font, &n.to_string(), x, y);
    }
}

/// Rasterise `text` with `fontdue` and blend it centred at `(cx, cy)`.
fn draw_str(pixmap: &mut Pixmap, font: &Font, text: &str, cx: f32, cy: f32) {
    // Rasterise every character.
    let glyphs: Vec<(Metrics, Vec<u8>)> =
        text.chars().map(|c| font.rasterize(c, FONT_PX)).collect();

    // Total advance width → used to centre the string horizontally.
    let total_advance: f32 = glyphs.iter().map(|(m, _)| m.advance_width).sum();

    // Highest ascender in the string → used to centre vertically.
    let ascent: f32 = glyphs
        .iter()
        .map(|(m, _)| (m.ymin + m.height as i32) as f32)
        .fold(0.0_f32, f32::max);

    // Cursor starts so the text block is centred at (cx, cy).
    let cursor_x = cx - total_advance / 2.0;
    // Baseline sits below centre by half the ascent.
    let baseline_y = cy + ascent / 2.0;

    let mut x = cursor_x;
    for (metrics, bitmap) in &glyphs {
        // Top-left corner of this glyph's bitmap in screen space.
        let bx = (x + metrics.xmin as f32).round() as i32;
        let by = (baseline_y - (metrics.ymin + metrics.height as i32) as f32).round() as i32;

        blend_black_glyph(pixmap, bitmap, metrics.width, metrics.height, bx, by);
        x += metrics.advance_width;
    }
}

/// Alpha-composite a black glyph's coverage bitmap onto `pixmap`.
/// Works for any background because it reads and modifies the existing pixels.
fn blend_black_glyph(
    pixmap: &mut Pixmap,
    bitmap: &[u8],
    glyph_w: usize,
    glyph_h: usize,
    ox: i32,
    oy: i32,
) {
    let pw = pixmap.width() as i32;
    let ph = pixmap.height() as i32;
    let data = pixmap.data_mut();

    for row in 0..glyph_h as i32 {
        for col in 0..glyph_w as i32 {
            let px = ox + col;
            let py = oy + row;
            if px < 0 || py < 0 || px >= pw || py >= ph {
                continue;
            }
            let coverage = bitmap[(row * glyph_w as i32 + col) as usize];
            if coverage == 0 {
                continue;
            }
            // Premultiplied-alpha blend of opaque black over the existing pixel.
            // src = (0, 0, 0, coverage); out_rgb = dst_rgb × (255 − coverage) / 255
            let a = coverage as u32;
            let inv = 255 - a;
            let i = (py * pw + px) as usize * 4;
            data[i]     = ((data[i]     as u32 * inv) / 255) as u8; // R
            data[i + 1] = ((data[i + 1] as u32 * inv) / 255) as u8; // G
            data[i + 2] = ((data[i + 2] as u32 * inv) / 255) as u8; // B
            // Alpha channel stays 255 (background is always fully opaque).
            data[i + 3] = (a + (data[i + 3] as u32 * inv) / 255) as u8;
        }
    }
}
