use bevy::prelude::*;

pub mod controller;

/// Система для обработки событий клавиатуры
pub fn kb_input_events(keyboard_input: Res<ButtonInput<KeyCode>>, mut windows: Query<&mut Window>) {
    if keyboard_input.just_pressed(KeyCode::Escape) {
        if let Ok(mut window) = windows.get_single_mut() {
            window.cursor_options.grab_mode = bevy::window::CursorGrabMode::None;
            window.cursor_options.visible = true;
        }
    }
}
