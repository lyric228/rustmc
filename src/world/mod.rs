mod chunk; // Module for managing chunks of the world
mod block; // Module for defining block types and properties
mod generation; // Module for world generation logic

use bevy::prelude::*;
use crate::world::block::{Block, VisualParams}; // Importing Block and VisualParams

pub struct WorldPlugin; // Plugin for managing world systems


impl Plugin for WorldPlugin {
    fn build(&self, app: &mut App) {
        app
            .add_systems(Startup, generate_blocks) // Adding block generation system
            .add_systems(Update, chunk::update_chunks); // Ensure chunk module is correctly referenced
    }
}

/// Function to generate blocks in front of the player
fn generate_blocks(mut commands: Commands) {
    let block_positions = vec![
        Vec3::new(0.0, 0.0, 0.0),
        Vec3::new(1.0, 0.0, 0.0),
        Vec3::new(2.0, 0.0, 0.0),
    ];

    for position in block_positions {
        commands.spawn(Block {
            name: "Basic Block".to_string(),
            tag: "block".to_string(),
            position,
            durability: 100.0,
            visual_params: VisualParams {
                texture: "path/to/texture.png".to_string(),
                color: Color::WHITE,
            },
            collision: true,
        });
    }
}
