use std::f32::consts::{FRAC_PI_2, TAU};
use chrono::{Local, Timelike};
use tiny_skia::{Paint, PathBuilder, Pixmap, Stroke, Transform};

/// Draw hour (50% × 3px), minute (75% × 2px), and second (90% × 1px) hands
/// from the current local wall-clock time.
pub fn draw_hands(pixmap: &mut Pixmap, cx: f32, cy: f32, radius: f32) {
    let now = Local::now();
    let h = now.hour() % 12;
    let m = now.minute();
    let s = now.second();
    let nanos = now.nanosecond();

    // Fractional components so hands move smoothly between ticks.
    let sec_f = s as f32 + nanos as f32 / 1_000_000_000.0;
    let min_f = m as f32 + sec_f / 60.0;
    let hour_f = h as f32 + min_f / 60.0;

    // 12 at the top (−π/2); each full sweep is one turn (TAU).
    let hour_angle = (hour_f / 12.0) * TAU - FRAC_PI_2;
    let minute_angle = (min_f / 60.0) * TAU - FRAC_PI_2;
    let second_angle = (sec_f / 60.0) * TAU - FRAC_PI_2;

    stroke_hand(pixmap, cx, cy, hour_angle, radius * 0.50, 3.0);
    stroke_hand(pixmap, cx, cy, minute_angle, radius * 0.75, 2.0);
    stroke_hand(pixmap, cx, cy, second_angle, radius * 0.90, 1.0);
}

fn stroke_hand(pixmap: &mut Pixmap, cx: f32, cy: f32, angle: f32, length: f32, width: f32) {
    let tip_x = cx + angle.cos() * length;
    let tip_y = cy + angle.sin() * length;

    let mut pb = PathBuilder::new();
    pb.move_to(cx, cy);
    pb.line_to(tip_x, tip_y);
    let path = pb.finish().expect("hand path");

    let mut paint = Paint::default();
    paint.set_color_rgba8(0, 0, 0, 255);

    let stroke = Stroke {
        width,
        ..Default::default()
    };

    pixmap.stroke_path(&path, &paint, &stroke, Transform::identity(), None);
}
