use macroquad::{input::{is_key_down, is_key_pressed}, shapes::draw_rectangle, time::get_frame_time, window::screen_height};

use crate::{LANE_KEYS, LANE_WIDTH, NOTE_CLICK_POS, NOTE_HEIGHT, NOTE_LENIENCE, NOTE_SPEED};

use super::lane::Lane;

#[derive(PartialEq, Clone, Debug)]
pub struct Note {
    pub pos: f32,
    pub lane: usize,
    pub spawn_time: f32,
}
impl Note {
    pub fn new(lane: usize, target_time: f32) -> Self {
        let note_click_pos = NOTE_CLICK_POS.lock().unwrap();

        let travel_time = NOTE_SPEED * get_frame_time() / *note_click_pos;
        // idk why but subtracting it by 1 second makes it more accurate...
        let spawn_time = target_time - travel_time - 1.0;

        std::mem::drop(note_click_pos);
        Note {pos: -NOTE_HEIGHT, lane, spawn_time}
    }
    
    pub fn render_note(&mut self, lanes: &Vec<Lane>) {
        draw_rectangle(lanes[self.lane].x_pos, self.pos, LANE_WIDTH, NOTE_HEIGHT, lanes[self.lane].color);
        self.pos += NOTE_SPEED * get_frame_time();
    }

    pub fn input_check (&mut self, lane_animation: &mut Vec<(bool, f32)>) -> bool {
        let note_click_pos = NOTE_CLICK_POS.lock().unwrap();
        
        if is_key_pressed(LANE_KEYS[self.lane]) {
            if self.pos >= *note_click_pos - NOTE_LENIENCE && self.pos <= *note_click_pos + NOTE_LENIENCE+NOTE_HEIGHT {
                std::mem::drop(note_click_pos); 
                lane_animation[self.lane].0 = true;
                return true
            }
            else {std::mem::drop(note_click_pos); return false}
        }else {std::mem::drop(note_click_pos); return false}
    }

    pub fn compare (&self, other_note: &Note) -> bool {
        self.pos == other_note.pos && self.lane == other_note.lane && self.spawn_time == other_note.spawn_time
    }
}