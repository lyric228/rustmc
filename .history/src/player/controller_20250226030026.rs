use bevy::prelude::*;
use super::Player;

pub fn movement(
    time: Res<Time>,
    keyboard_input: Res<Input<KeyCode>>,
    mut query: Query<(&mut Transform, &Player)>,
) {
    for (mut transform, player) in query.iter_mut() {
        let mut direction = Vec3::ZERO;
        
        if keyboard_input.pressed(KeyCode::KeyW) {
            direction.z -= 1.0;
        }
        if keyboard_input.pressed(KeyCode::KeyS) {
            direction.z += 1.0;
        }
        if keyboard_input.pressed(KeyCode::KeyA) {
            direction.x -= 1.0;
        }
        if keyboard_input.pressed(KeyCode::KeyD) {
            direction.x += 1.0;
        }

        direction = direction.normalize_or_zero();
        transform.translation += direction * player.movement_speed * time.delta_seconds();
    }
}

pub fn mouse_input(
    time: Res<Time>,
    mut mouse_motion: EventReader<MouseMotion>,
    mut query: Query<(&mut Transform, &Player)>,
) {
    let mut delta = Vec2::ZERO;
    for event in mouse_motion.read() {
        delta += event.delta;
    }

    for (mut transform, player) in query.iter_mut() {
        transform.rotate_y(-delta.x * player.mouse_sensitivity * time.delta_seconds());
    }
}

pub fn block_interaction(
    mouse_input: Res<Input<MouseButton>>,
    windows: Query<&Window>,
    camera_query: Query<(&Camera, &GlobalTransform)>,
) {
    if mouse_input.just_pressed(MouseButton::Left) {
        // Реализация взаимодействия с блоками
        if let Some(cursor_position) = windows.single().cursor_position() {
            let (camera, camera_transform) = camera_query.single();
            if let Some(ray) = camera.viewport_to_world(camera_transform, cursor_position) {
                // Реализация ray casting для взаимодействия с блоками
            }
        }
    }
}
