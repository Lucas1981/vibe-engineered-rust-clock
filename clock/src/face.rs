use crate::layout::{BACKGROUND, FACE_STROKE_WIDTH, INK};
use tiny_skia::{Paint, PathBuilder, Pixmap, Stroke, Transform};

/// Fill the pixmap with white, then stroke a black circle outline.
/// `cx`, `cy` are the centre coordinates; `radius` is to the middle of the stroke.
pub fn draw_face(pixmap: &mut Pixmap, cx: f32, cy: f32, radius: f32) {
    pixmap.fill(BACKGROUND);

    let path = PathBuilder::from_circle(cx, cy, radius).expect("circle path");

    let mut paint = Paint::default();
    paint.set_color(INK);

    let stroke = Stroke {
        width: FACE_STROKE_WIDTH,
        ..Default::default()
    };

    pixmap.stroke_path(&path, &paint, &stroke, Transform::identity(), None);
}
