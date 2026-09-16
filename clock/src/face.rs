use tiny_skia::{Color, Paint, PathBuilder, Pixmap, Stroke, Transform};

/// Fill the pixmap with white, then stroke a black circle outline.
/// `cx`, `cy` are the centre coordinates; `radius` is to the middle of the stroke.
pub fn draw_face(pixmap: &mut Pixmap, cx: f32, cy: f32, radius: f32) {
    // --- white background ---
    pixmap.fill(Color::WHITE);

    // --- black circle outline ---
    let path = PathBuilder::from_circle(cx, cy, radius).expect("circle path");

    let mut paint = Paint::default();
    paint.set_color_rgba8(0, 0, 0, 255); // opaque black

    let stroke = Stroke {
        width: 4.0,
        ..Default::default()
    };

    pixmap.stroke_path(&path, &paint, &stroke, Transform::identity(), None);
}
