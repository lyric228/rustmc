use bevy::prelude::*;
use super::Player;

pub fn spawn_camera(mut commands: Commands) {
    commands.spawn((
        Camera3dBundle {
            transform: Transform::from_xyz(-2.0, 2.5, 5.0)
                .looking_at(Vec3::ZERO, Vec3::Y),
            ..default()
        },
        Player {
            movement_speed: 5.0,
            mouse_sensitivity: 0.5,
        }
    ));
}
