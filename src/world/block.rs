use bevy::prelude::*;

#[derive(Debug, Clone, Component)]
pub struct VisualParams {
    pub texture: String,
    pub color: Color,
}

#[derive(Component)]
pub struct Block {
    pub name: String,
    pub tag: String,
    pub collision: bool,
}

#[derive(Bundle)]
pub struct BlockBundle {
    pub block: Block,
    pub visual: VisualParams,
    pub transform: Transform,
    pub global_transform: GlobalTransform,
}

impl BlockBundle {
    pub fn new(
        name: impl Into<String>,
        tag: impl Into<String>,
        position: Vec3,
        visual_params: VisualParams,
        collision: bool,
    ) -> Self {
        Self {
            block: Block {
                name: name.into(),
                tag: tag.into(),
                collision,
            },
            visual: visual_params,
            transform: Transform::from_translation(position),
            global_transform: GlobalTransform::default(),
        }
    }
}

pub struct BlockPlugin;

impl Plugin for BlockPlugin {
    fn build(&self, app: &mut App) {}
}

pub mod block_helpers {
    use super::*;

    pub fn spawn_block(
        commands: &mut Commands,
        name: impl Into<String>,
        tag: impl Into<String>,
        position: Vec3,
        visual_params: VisualParams,
        collision: bool,
    ) -> Entity {
        commands.spawn(BlockBundle::new(
            name,
            tag,
            position,
            visual_params,
            collision,
        )).id()
    }

    pub fn delete_block(commands: &mut Commands, entity: Entity) {
        commands.entity(entity).despawn_recursive();
    }

    pub fn set_block_position(
        commands: &mut Commands,
        entity: Entity,
        new_position: Vec3,
    ) {
        commands.entity(entity).insert(Transform::from_translation(new_position));
    }
}
