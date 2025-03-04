mod chunk;
mod block;
mod generation;
use bevy::prelude::*;
use crate::world::block::{block_helpers, VisualParams};

pub struct WorldPlugin;
impl Plugin for WorldPlugin {
    fn build(&self, app: &mut App) {
        app
            .add_systems(Startup, generate_blocks)
            .add_systems(Update, chunk::update_chunks);
    }
}

fn generate_blocks(mut commands: Commands) {
    let block_positions = vec![
        Vec3::new(0.0, 0.0, -2.0),
        Vec3::new(1.0, 0.0, -2.0),
        Vec3::new(-1.0, 0.0, -2.0),
        Vec3::new(0.0, 1.0, -2.0),
        Vec3::new(0.0, -1.0, -2.0),
    ];

    for position in block_positions {
        let visual = VisualParams {
            texture: "textures/block/cobblestone.png".to_string(),
            color: Color::WHITE,
        };

        let entity = block_helpers::spawn_block(
            &mut commands,
            "Cobblestone",
            "cobblestone",
            position,
            visual,
            true,
        );

        commands.entity(entity).insert(Transform {
            scale: Vec3::splat(0.3),
            ..Transform::from_translation(position)
        });
    }
}
