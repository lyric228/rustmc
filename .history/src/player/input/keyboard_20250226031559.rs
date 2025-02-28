use bevy::{
    input::mouse::{AccumulatedMouseMotion, AccumulatedMouseScroll},
    prelude::*,
};


pub fn kb_input_events(kb_input: Res<ButtonInput<KeyCode>>) {
    if kb_input.pressed(KeyCode::KeyA) {
        info!("'A' currently pressed");
    }

    if kb_input.just_pressed(KeyCode::KeyA) {
        info!("'A' just pressed");
    }
    if kb_input.just_released(KeyCode::KeyA) {
        info!("'A' just released");
    }
}
