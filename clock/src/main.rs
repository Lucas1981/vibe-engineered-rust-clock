mod canvas;
mod face;
mod hands;
mod layout;
mod text;

use canvas::Canvas;
use fontdue::{Font, FontSettings};
use layout::{CX, CY, HEIGHT, RADIUS, WIDTH};

fn main() {
    // Load the font once — rasterisation happens per-frame but parsing is not.
    let font_bytes: &[u8] = include_bytes!("../assets/liberation-sans-bold.ttf");
    let font = Font::from_bytes(font_bytes, FontSettings::default()).expect("failed to load font");

    let mut canvas = Canvas::new(WIDTH, HEIGHT, "Clock");

    while canvas.is_open() {
        let pix = canvas.pixmap_mut();
        face::draw_face(pix, CX, CY, RADIUS);
        text::draw_numerals(pix, &font, CX, CY, RADIUS);
        hands::draw_hands(pix, CX, CY, RADIUS);
        canvas.present();
    }
}
