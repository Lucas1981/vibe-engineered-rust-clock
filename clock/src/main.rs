mod canvas;
mod face;

use canvas::Canvas;

const WIDTH: u32 = 600;
const HEIGHT: u32 = 600;
const CX: f32 = WIDTH as f32 / 2.0;
const CY: f32 = HEIGHT as f32 / 2.0;
const RADIUS: f32 = 270.0; // leaves a 30 px margin on each side

fn main() {
    let mut canvas = Canvas::new(WIDTH, HEIGHT, "Clock");

    while canvas.is_open() {
        face::draw_face(canvas.pixmap_mut(), CX, CY, RADIUS);
        canvas.present();
    }
}
