use bevy::{
    input::mouse::{AccumulatedMouseMotion, AccumulatedMouseScroll},
    window::CursorGrabMode,
    prelude::*,
};


pub fn mouse_click_system(mouse_button_input: Res<ButtonInput<MouseButton>>) {
    if mouse_button_input.pressed(MouseButton::Left) {
        info!("left mouse currently pressed");
    }

    if mouse_button_input.just_pressed(MouseButton::Left) {
        info!("left mouse just pressed");
    }

    if mouse_button_input.just_released(MouseButton::Left) {
        info!("left mouse just released");
    }
}

pub fn mouse_move_system(
    accumulated_mouse_motion: Res<AccumulatedMouseMotion>,
    accumulated_mouse_scroll: Res<AccumulatedMouseScroll>,
) {
    if accumulated_mouse_motion.delta != Vec2::ZERO {
        let delta = accumulated_mouse_motion.delta;
        info!("mouse moved ({}, {})", delta.x, delta.y);
    }
    if accumulated_mouse_scroll.delta != Vec2::ZERO {
        let delta = accumulated_mouse_scroll.delta;
        info!("mouse scrolled ({}, {})", delta.x, delta.y);
    }
}


pub fn grab_mouse(
    mut window: Single<&mut Window>,
    mouse: Res<ButtonInput<MouseButton>>,
    key: Res<ButtonInput<KeyCode>>,
) {
    if mouse.just_pressed(MouseButton::Left) {
        window.cursor_options.visible = false;
        window.cursor_options.grab_mode = CursorGrabMode::Locked;
    }

    if key.just_pressed(KeyCode::Escape) {
        window.cursor_options.visible = true;
        window.cursor_options.grab_mode = CursorGrabMode::None;
    }
}
