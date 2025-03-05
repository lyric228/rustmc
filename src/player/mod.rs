pub mod controller;
pub mod camera; 
pub mod types;

use crate::player::controller::*;
use crate::player::types::*;
use bevy::prelude::*;

pub struct PlayerPlugin;

impl Plugin for PlayerPlugin {
    fn build(&self, app: &mut App) {
        app
            .add_systems(Startup, (
                |commands: Commands, asset_server: Res<AssetServer>| {
                    spawn_text_default(commands, asset_server, 25.0, "текст");
                },
                spawn_player,
            ))
            .add_systems(Update, ( 
                controller::mouse_click_system,
                controller::mouse_move_system,
                controller::grab_mouse,
                controller::kb_input_events,
            ));
    }
}


#[derive(Component)]
pub struct Player {
    pub movement_speed: f32,
    pub mouse_sensitivity: f32,
}
