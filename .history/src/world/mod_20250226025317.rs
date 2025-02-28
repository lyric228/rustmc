mod chunk;
mod block;
mod generation;

use bevy::prelude::*;
use sysx::io::fs;

pub struct WorldPlugin;

impl Plugin for WorldPlugin {
    fn build(&self, app: &mut App) {
        app
            .add_systems(Startup, generation::generate_terrain)
            .add_systems(Update, chunk::update_chunks);
    }
}

#[derive(Component)]
pub struct Block {
    pub block_type: BlockType,
    pub durability: f32,
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum BlockType {
    Air,
    Grass,
    Dirt,
    Stone,
}
