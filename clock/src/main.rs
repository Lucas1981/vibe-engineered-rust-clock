mod canvas;
mod face;
mod text;

use canvas::Canvas;
use fontdue::{Font, FontSettings};

const WIDTH: u32 = 600;
const HEIGHT: u32 = 600;
const CX: f32 = WIDTH as f32 / 2.0;
const CY: f32 = HEIGHT as f32 / 2.0;
const RADIUS: f32 = 270.0; // leaves a 30 px margin on each side

fn main() {
    // Load the font once — rasterisation happens per-frame but parsing is not.
    let font_bytes = include_bytes!("../assets/arial-bold.ttf");
    let font = Font::from_bytes(font_bytes as &[u8], FontSettings::default())
        .expect("failed to load font");

    let mut canvas = Canvas::new(WIDTH, HEIGHT, "Clock");

    while canvas.is_open() {
        let pix = canvas.pixmap_mut();
        face::draw_face(pix, CX, CY, RADIUS);
        text::draw_numerals(pix, &font, CX, CY, RADIUS);
        canvas.present();
    }
}
