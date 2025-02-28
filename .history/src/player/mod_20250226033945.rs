pub mod controller;
pub mod camera;
pub mod types;

use bevy::prelude::*;

pub struct PlayerPlugin;

impl Plugin for PlayerPlugin {
    fn build(&self, app: &mut App) {
        app
            .add_systems(Startup, camera::spawn_camera)
            .add_systems(Update, (
                controller::grab_mouse,
                controller::kb_input_events,
                controller::mouse_click_system,
                controller::mouse_move_system,
                //controller::
            ));
    }
}

#[derive(Component)]
pub struct Player {
    pub movement_speed: f32,
    pub mouse_sensitivity: f32,
}
