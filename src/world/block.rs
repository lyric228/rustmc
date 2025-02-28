use bevy::prelude::*;

#[derive(Component)]
pub struct Block {
    pub name: String,
    pub tag: String,
    pub position: Vec3,
    pub durability: f32,
    pub visual_params: VisualParams,
    pub collision: bool, 
}


impl Block {
    pub fn new(commands: &mut Commands, name: String, tag: String, position: Vec3, durability: f32, visual_params: VisualParams, collision: bool) {
        commands.spawn(Block {
            name,
            tag,
            position,
            durability,
            visual_params,
            collision,
        });
    }

    pub fn delete(commands: &mut Commands, entity: Entity) {
        commands.entity(entity).despawn();
    }

    pub fn set_position(&mut self, new_position: Vec3) {
        self.position = new_position;
    }

    pub fn set_durability(&mut self, new_durability: f32) {
        self.durability = new_durability;
    }

    pub fn set_visual_params(&mut self, new_visual_params: VisualParams) {
        self.visual_params = new_visual_params;
    }
}

#[derive(Debug, Clone)]
pub struct VisualParams {
    pub texture: String,
    pub color: Color,
}
