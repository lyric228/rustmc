// Файл со специфичной логикой контроллера клавиатуры, если требуется
use bevy::prelude::*;

/// Handles keyboard input events
pub fn kb_input_events(kb_input: Res<ButtonInput<KeyCode>>) {
    if kb_input.just_pressed(KeyCode::KeyW) {
        info!("pressed W");
    }
    if kb_input.just_pressed(KeyCode::KeyS) {
        info!("pressed S");
    }
    if kb_input.just_pressed(KeyCode::KeyA) {
        info!("pressed A");
    }
    if kb_input.just_pressed(KeyCode::KeyD) {
        info!("pressed D");
    }
}
