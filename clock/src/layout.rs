use tiny_skia::Color;

pub const WIDTH: u32 = 600;
pub const HEIGHT: u32 = 600;
pub const MARGIN: f32 = 30.0;

pub const CX: f32 = WIDTH as f32 / 2.0;
pub const CY: f32 = HEIGHT as f32 / 2.0;
pub const RADIUS: f32 = WIDTH as f32 / 2.0 - MARGIN;

pub const BACKGROUND: Color = Color::WHITE;
pub const INK: Color = Color::BLACK;

pub const FACE_STROKE_WIDTH: f32 = 4.0;

pub const NUMERAL_RADIUS_FRAC: f32 = 0.78;
pub const NUMERAL_FONT_PX: f32 = 30.0;

pub const HOUR_HAND_LENGTH_FRAC: f32 = 0.50;
pub const MINUTE_HAND_LENGTH_FRAC: f32 = 0.75;
pub const SECOND_HAND_LENGTH_FRAC: f32 = 0.90;
pub const HOUR_HAND_WIDTH: f32 = 3.0;
pub const MINUTE_HAND_WIDTH: f32 = 2.0;
pub const SECOND_HAND_WIDTH: f32 = 1.0;
