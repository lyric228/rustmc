use bevy::{input::mouse::MouseButtonInput, prelude::*};
use super::Player;

fn kb_input_events(keyboard_input: Res<ButtonInput<KeyCode>>) {
    if keyboard_input.pressed(KeyCode::KeyA) {
        info!("'A' currently pressed");
    }

    if keyboard_input.just_pressed(KeyCode::KeyA) {
        info!("'A' just pressed");
    }
    if keyboard_input.just_released(KeyCode::KeyA) {
        info!("'A' just released");
    }
}
