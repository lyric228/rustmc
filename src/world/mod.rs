mod chunk;
mod block;
mod generation;
use bevy::prelude::*;

pub struct WorldPlugin;

impl Plugin for WorldPlugin {
    fn build(&self, app: &mut App) {
        app
            .add_plugins(block::BlocksPlugin)
            .add_systems(Startup, generate_blocks)
            .add_systems(Update, chunk::update_chunks);
    }
}

fn generate_blocks(
    mut commands: Commands,
    mut block_registry: ResMut<block::BlockRegistry>,
    mut materials: ResMut<Assets<StandardMaterial>>,
    _meshes: ResMut<Assets<Mesh>>, // Prefix with underscore to mark as intentionally unused
    asset_server: Res<AssetServer>,
) {
    // Create a material with the cobblestone texture
    let cobblestone_texture = asset_server.load("textures/block/cobblestone.png");
    let cobblestone_material = materials.add(StandardMaterial {
        base_color_texture: Some(cobblestone_texture),
        ..default()
    });

    // Define block positions
    let block_positions = [
        (0, 0, -2),
        (0, 1, -2),
        (0, 2, -2),
        (0, 3, -2),
        (0, 4, -2),
    ];

    // Create blocks at the specified positions
    for (x, y, z) in block_positions {
        block_registry.create_block(x, y, z, cobblestone_material.clone(), 0.5);
    }

    // Add a light so we can see the blocks - using updated API
    commands.spawn((
        PointLight {
            intensity: 1500.0,
            shadows_enabled: true,
            ..default()
        },
        Transform::from_xyz(4.0, 8.0, 4.0),
    ));
}
