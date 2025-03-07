use bevy::prelude::*;
use bevy::input::mouse::MouseMotion;
use crate::player::types::*;
use bevy::window::CursorGrabMode;

/// Handles mouse click events
pub fn mouse_click_system(
    buttons: Res<ButtonInput<MouseButton>>,
    mut windows: Query<&mut Window>
) {
    if buttons.just_pressed(MouseButton::Left) {
        if let Ok(mut window) = windows.get_single_mut() {
            window.cursor_options.grab_mode = CursorGrabMode::Locked;
            window.cursor_options.visible = false;
        }
    }

    if buttons.just_pressed(MouseButton::Right) {
        if let Ok(mut window) = windows.get_single_mut() {
            window.cursor_options.grab_mode = CursorGrabMode::None;
            window.cursor_options.visible = true;
        }
    }
}

/// Handles mouse movement for player look control
pub fn mouse_move_system(
    mut mouse_motion_events: EventReader<MouseMotion>,
    mut query: Query<(&mut PlayerLook, &mut Transform), With<Player>>,
) {
    // Sensitivity configuration
    const MOUSE_SENSITIVITY: f32 = 0.002;

    // Accumulated mouse movement for this frame
    let mut delta = Vec2::ZERO;
    for event in mouse_motion_events.read() {
        delta += event.delta;
    }

    // Skip if no movement
    if delta.is_nan() || (delta.x == 0.0 && delta.y == 0.0) {
        return;
    }

    // Apply rotation to player entities
    for (mut look, mut transform) in query.iter_mut() {
        // Calculate rotation changes
        let delta_yaw = -delta.x * MOUSE_SENSITIVITY;
        let delta_pitch = -delta.y * MOUSE_SENSITIVITY;

        // Update rotation angles
        look.yaw += delta_yaw;
        look.pitch += delta_pitch;

        // Clamp pitch to prevent over-rotation
        look.pitch = look.pitch.clamp(-1.5, 1.5); // Approximately -85° to 85°

        // Apply horizontal rotation to transform
        let yaw_rotation = Quat::from_rotation_y(look.yaw);
        transform.rotation = yaw_rotation;
    }
}

/// Grabs the mouse cursor for better camera control
pub fn grab_mouse(
    mut windows: Query<&mut Window>
) {
    if let Ok(mut window) = windows.get_single_mut() {
        window.cursor_options.visible = false;
        window.cursor_options.grab_mode = bevy::window::CursorGrabMode::Locked;
    }
}
