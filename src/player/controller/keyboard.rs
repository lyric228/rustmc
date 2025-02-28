use bevy::prelude::*;


pub fn kb_input_events(kb_input: Res<ButtonInput<KeyCode>>) {
    if kb_input.pressed(KeyCode::KeyW) {
        info!("'W' currently pressed");
    }

    if kb_input.just_pressed(KeyCode::KeyW) {
        info!("'W' just pressed");
    }
    if kb_input.just_released(KeyCode::KeyW) {
        info!("'W' just released");
    }

    if kb_input.pressed(KeyCode::KeyA) {
        info!("'A' currently pressed");
    }

    if kb_input.just_pressed(KeyCode::KeyA) {
        info!("'A' just pressed");
    }
    if kb_input.just_released(KeyCode::KeyA) {
        info!("'A' just released");
    }

    if kb_input.pressed(KeyCode::KeyS) {
        info!("'S' currently pressed");
    }

    if kb_input.just_pressed(KeyCode::KeyS) {
        info!("'S' just pressed");
    }
    if kb_input.just_released(KeyCode::KeyS) {
        info!("'S' just released");
    }

    if kb_input.pressed(KeyCode::KeyD) {
        info!("'D' currently pressed");
    }

    if kb_input.just_pressed(KeyCode::KeyD) {
        info!("'D' just pressed");
    }
    if kb_input.just_released(KeyCode::KeyD) {
        info!("'D' just released");
    }
}
