use bevy::prelude::*;
use std::collections::HashMap;

pub struct Block {
    pub position: Vec3,
    pub size: f32,
    pub material: Handle<StandardMaterial>,
    pub entity: Option<Entity>,
}

impl Default for Block {
    fn default() -> Self {
        Self {
            position: Vec3::ZERO,
            size: 1.0,
            material: Handle::default(),
            entity: None,
        }
    }
}

pub struct BlocksPlugin;

impl Plugin for BlocksPlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<BlockRegistry>()
            .add_systems(Update, update_blocks);
    }
}

#[derive(Resource, Default)]
pub struct BlockRegistry {
    blocks: HashMap<(i32, i32, i32), Block>,
}

impl BlockRegistry {
    pub fn create_block(
        &mut self,
        x: i32,
        y: i32,
        z: i32,
        material: Handle<StandardMaterial>,
        size: f32,
    ) -> &mut Block {
        let position = Vec3::new(x as f32, y as f32, z as f32);
        let block = Block {
            position,
            size,
            material,
            entity: None,
        };
        self.blocks.insert((x, y, z), block);
        self.blocks.get_mut(&(x, y, z)).unwrap()
    }

    pub fn get_block(&self, x: i32, y: i32, z: i32) -> Option<&Block> {
        self.blocks.get(&(x, y, z))
    }

    pub fn get_block_mut(&mut self, x: i32, y: i32, z: i32) -> Option<&mut Block> {
        self.blocks.get_mut(&(x, y, z))
    }

    pub fn remove_block(&mut self, x: i32, y: i32, z: i32) -> Option<Block> {
        self.blocks.remove(&(x, y, z))
    }

    pub fn all_blocks(&self) -> impl Iterator<Item = &Block> {
        self.blocks.values()
    }
}

fn update_blocks(
    mut commands: Commands,
    mut block_registry: ResMut<BlockRegistry>,
    mut meshes: ResMut<Assets<Mesh>>,
) {
    for (_, block) in block_registry.blocks.iter_mut() {
        if block.entity.is_none() {
            let translation = block.position;
            let size = block.size;

            let entity = commands
                .spawn((
                    Mesh3d(meshes.add(Mesh::from(bevy::math::prelude::Cuboid {
                        half_size: Vec3::splat(1.0),
                    }))),
                    MeshMaterial3d(block.material.clone()),
                    Transform {
                        translation,
                        scale: Vec3::splat(size),
                        ..Default::default()
                    },
                    GlobalTransform::default(),
                    Visibility::default(),
                    InheritedVisibility::default(),
                    ViewVisibility::default(),
                ))
                .id();

            block.entity = Some(entity);
        }
    }
}
