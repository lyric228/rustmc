mod controller;
mod camera;

use bevy::prelude::*;

pub struct PlayerPlugin;

impl Plugin for PlayerPlugin {
    fn build(&self, app: &mut App) {
        app
            .add_systems(Startup, camera::spawn_camera)
            .add_systems(Update, (
                controller::grab_mouse;,
                controller::mouse_input_events,
            ));
    }
}

#[derive(Component)]
pub struct Player {
    pub movement_speed: f32,
    pub mouse_sensitivity: f32,
}
