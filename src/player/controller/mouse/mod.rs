use bevy::prelude::*;
use bevy::window::CursorGrabMode;

/// Система для обработки нажатий мыши
pub fn mouse_click_system(buttons: Res<ButtonInput<MouseButton>>, mut windows: Query<&mut Window>) {
    if buttons.just_pressed(MouseButton::Left) {
        if let Ok(mut window) = windows.get_single_mut() {
            window.cursor.grab_mode = CursorGrabMode::Locked;
            window.cursor.visible = false;
        }
    }

    if buttons.just_pressed(MouseButton::Right) {
        if let Ok(mut window) = windows.get_single_mut() {
            window.cursor.grab_mode = CursorGrabMode::None;
            window.cursor.visible = true;
        }
    }
}

/// Система для захвата курсора мыши при запуске
pub fn grab_mouse(mut windows: Query<&mut Window>) {
    if let Ok(mut window) = windows.get_single_mut() {
        window.cursor.grab_mode = CursorGrabMode::Locked;
        window.cursor.visible = false;
    }
}
