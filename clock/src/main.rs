mod canvas;

use canvas::Canvas;

const WIDTH: u32 = 600;
const HEIGHT: u32 = 600;

fn main() {
    let mut canvas = Canvas::new(WIDTH, HEIGHT, "Clock");

    while canvas.is_open() {
        canvas.present();
    }
}
