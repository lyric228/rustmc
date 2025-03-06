use crate::player::types::*;
use bevy::prelude::*;

mod controller;

pub use controller::*;

/// Система для обработки событий клавиатуры
pub fn kb_input_events(keyboard_input: Res<ButtonInput<KeyCode>>, mut windows: Query<&mut Window>) {
    if keyboard_input.just_pressed(KeyCode::Escape) {
        if let Ok(mut window) = windows.get_single_mut() {
            window.cursor.grab_mode = bevy::window::CursorGrabMode::None;
            window.cursor.visible = true;
        }
    }
}
