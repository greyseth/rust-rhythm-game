mod models{pub mod note; pub mod lane;}

use std::{f32::INFINITY, sync::Mutex, thread};

use macroquad::{color::{Color, BLACK, BLUE, GREEN, ORANGE, PURPLE, RED, WHITE}, input::{is_key_down, is_key_pressed, KeyCode}, math, shapes::{draw_line, draw_rectangle}, text::draw_text, time::get_frame_time, window::{clear_background, next_frame, screen_height, screen_width}};
use models::{lane::Lane, note::{self, Note}};
use once_cell::sync::Lazy;

pub const LANE_COLORS: [Color; 4] = [BLUE, RED, GREEN, ORANGE];
pub const LANE_KEYS: [KeyCode; 4] = [KeyCode::D, KeyCode::F, KeyCode::J, KeyCode::K];

pub const LANE_WIDTH: f32 = 100.0;
pub const LANE_MARGIN: f32 = 5.0;
pub const NOTE_HEIGHT: f32 = 15.0;
pub const NOTE_SPEED: f32 = 500.0;
const NOTE_LENIENCE: f32 = 25.0;
pub const NOTE_CLICK_POS_PERCENTAGE: f32 = 10.0;

pub static NOTE_CLICK_POS: Lazy<Mutex<f32>> = Lazy::new(|| Mutex::new(0.0));

#[macroquad::main("Rhythm Game")]
async fn main() {
    // (isAnimating, elapsed_time)
    let mut lane_animation: Vec<(bool, f32)> = Vec::new();
    
    let mut lanes: Vec<Lane> = Vec::new();
    let mut notes: Vec<Note> = Vec::new();
    let mut spawned_notes: Vec<Note> = Vec::new();

    let mut time_elapsed = 0.0;

    let mut note_click_pos = NOTE_CLICK_POS.lock().unwrap();
    *note_click_pos = screen_height() - (screen_height() * (NOTE_CLICK_POS_PERCENTAGE / 100.0));
    std::mem::drop(note_click_pos);

    testing_notes(&mut notes);
    
    loop {
        clear_background(BLACK);

        draw_text(format!("{}", time_elapsed).as_str(), 0.0, 25.0, 25.0, WHITE);
        draw_text(format!("{:?}", notes).as_str(), 0.0, 35.0, 10.0, WHITE);
        draw_text(format!("{:?}", spawned_notes).as_str(), 0.0, 45.0, 10.0, WHITE);
        
        // Creates white background for lanes
        draw_rectangle(screen_width() / 2.0 - (LANE_WIDTH * (LANE_COLORS.len() as f32/2.0) + LANE_MARGIN), 0.0, LANE_WIDTH * LANE_COLORS.len() as f32 + LANE_MARGIN * 2.0, screen_height(), WHITE);
        
        // Initializes lanes
        lanes.clear();
        let mut note_click_pos = NOTE_CLICK_POS.lock().unwrap();

        *note_click_pos = screen_height() - (screen_height() * (NOTE_CLICK_POS_PERCENTAGE / 100.0));
        lanes.clear();
        for (i, &color) in LANE_COLORS.iter().enumerate() {
            let x_pos = screen_width() / 2.0 + LANE_WIDTH * (i as f32 - 2.0);
            draw_rectangle(x_pos, *note_click_pos, LANE_WIDTH, NOTE_HEIGHT, color);
            lanes.push(Lane::new(x_pos, color));

            if lane_animation.len() < LANE_COLORS.len() {lane_animation.push((false, 0.0));}
        }
        draw_line(lanes[0].x_pos, *note_click_pos-NOTE_LENIENCE, lanes[lanes.len()-1 as usize].x_pos + LANE_WIDTH, *note_click_pos-NOTE_LENIENCE, 2.0, PURPLE);
        draw_line(lanes[0].x_pos, *note_click_pos+NOTE_LENIENCE+NOTE_HEIGHT, lanes[lanes.len()-1 as usize].x_pos + LANE_WIDTH, *note_click_pos+NOTE_LENIENCE+NOTE_HEIGHT, 2.0, PURPLE);
        std::mem::drop(note_click_pos);

        // Handles lane animation
        for (index, la) in lane_animation.iter_mut().enumerate() {
            lanes[index].animate(la);
        }

        // Creates notes based on elapsed time
        let mut spawn_indexes: Vec<Note> = Vec::new();
        for note in notes.iter() {
            if time_elapsed >= note.spawn_time {
                spawned_notes.push(note.clone());
                spawn_indexes.push(note.clone());
            }
        }

        for note_remove in spawn_indexes {notes.remove(notes.iter().position(|n| n.compare(&note_remove)).unwrap());}
        
        // Handles spawned notes
        let mut remove_indexes: Vec<Note> = Vec::new();
        for note in spawned_notes.iter_mut() {
            note.render_note(&lanes);
            if note.input_check(&mut lane_animation) {remove_indexes.push(note.clone());}

            if note.pos >= screen_height() {remove_indexes.push(note.clone());}
        }

        for note_remove in remove_indexes {spawned_notes.remove(spawned_notes.iter().position(|n| n.compare(&note_remove)).unwrap());}

        time_elapsed += get_frame_time();

        if is_key_pressed(KeyCode::LeftControl) && is_key_down(KeyCode::R) {testing_notes(&mut notes);}

        next_frame().await;
    }
}

fn testing_notes(notes: &mut Vec<Note>) {
    notes.push(Note::new(0, 1.0));
    notes.push(Note::new(1, 1.5));
    notes.push(Note::new(0, 2.5));
    notes.push(Note::new(2, 3.0));
    notes.push(Note::new(3, 3.0));
}