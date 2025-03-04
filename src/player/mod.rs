pub mod controller; // Player control logic
pub mod camera; // Camera management
pub mod types; // Player-related types

use crate::player::controller::*; // Importing player control systems
use crate::player::types::*; // Importing player-related types
use bevy::prelude::*;

pub struct PlayerPlugin; // Plugin for managing player systems

impl Plugin for PlayerPlugin { // Implementing the PlayerPlugin
    fn build(&self, app: &mut App) {
        app
            .add_systems(Startup, (spawn_text, spawn_player))
            .add_systems(Update, ( 
                controller::mouse_click_system, // Added mouse click system
                controller::mouse_move_system,   // Added mouse move system
                controller::grab_mouse,
                controller::kb_input_events,
                controller::mouse_click_system,
                controller::mouse_move_system,
            ));
    }
}


#[derive(Component)]
pub struct Player {
    pub movement_speed: f32,
    pub mouse_sensitivity: f32,
}
