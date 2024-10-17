use macroquad::color::Color;

pub struct Lane {
    pub x_pos: f32,
    pub color: Color
}
impl Lane {
    pub fn new(x_pos: f32, color: Color) -> Self {
        Lane {x_pos, color}
    }
}