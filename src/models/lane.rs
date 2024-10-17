use macroquad::{color::Color, math::Vec2, shapes::draw_rectangle, time::get_frame_time};

use crate::{LANE_WIDTH, NOTE_CLICK_POS, NOTE_HEIGHT};

const ANIM_SCALE_FACTOR: f32 = 1.5;
const ANIM_TIME: f32 = 0.3;

pub struct Lane {
    pub x_pos: f32,
    pub color: Color,
}
impl Lane {
    pub fn new(x_pos: f32, color: Color) -> Self {
        Lane {x_pos, color}
    }

    pub fn animate(&mut self, data: &mut (bool, f32)) {
        // data: (isAnimating, animationTime);
        if !data.0 {return;}
        if data.1 > ANIM_TIME {data.0 = false; data.1 = 0.0; return;}
        
        let note_click_pos = NOTE_CLICK_POS.lock().unwrap();
        
        let anim_progress = data.1 / ANIM_TIME;
        let scale_factor = ANIM_SCALE_FACTOR * anim_progress;
        
        // Gets vertex coordinates of original size
        let mut v_pos: Vec<Vec2> = Vec::new();
        v_pos.push(Vec2::new(self.x_pos, *note_click_pos)); // top left
        v_pos.push(Vec2::new(self.x_pos, *note_click_pos+NOTE_HEIGHT)); // bottom left
        v_pos.push(Vec2::new(self.x_pos+LANE_WIDTH, *note_click_pos)); // top right
        v_pos.push(Vec2::new(self.x_pos+LANE_WIDTH, *note_click_pos+NOTE_HEIGHT)); // bottom right

        // Repositions each vertex by scale factor based on center point
        let mut center_point: Vec2 = Vec2::new((v_pos[0].x + v_pos[2].x) / 2.0, (v_pos[0].y + v_pos[1].y) / 2.0);
        for pos in v_pos.iter_mut() {
            *pos = Vec2::new(
                center_point.x + (pos.x - center_point.x) * scale_factor, // Scale horizontally
                center_point.y + (pos.y - center_point.y) * scale_factor  // Scale vertically
            );
        }

        // Draws the rectangle using scaled coordinates
        draw_rectangle(v_pos[0].x, v_pos[0].y, (v_pos[2].x - v_pos[0].x), (v_pos[1].y - v_pos[0].y), self.color);

        data.1 += get_frame_time();
        std::mem::drop(note_click_pos);
    }
}