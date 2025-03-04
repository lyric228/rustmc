use bevy::prelude::*;

pub const DEFAULT_RENDER_LAYER: usize = 0;
pub const VIEW_MODEL_RENDER_LAYER: usize = 1;

#[derive(Debug, Component)]
pub struct Player;

#[derive(Debug, Component, Deref, DerefMut)]
pub struct CameraSensitivity(Vec2);

impl Default for CameraSensitivity {
    fn default() -> Self {
        Self(  
            Vec2::new(0.003, 0.002),
        )
    }
}

#[derive(Debug, Component)]
pub struct WorldModelCamera;

#[derive(Debug, Component, Clone, Copy, PartialEq, Default, Deref, DerefMut)]
pub struct AccumulatedInput(Vec2);

#[derive(Debug, Component, Clone, Copy, PartialEq, Default, Deref, DerefMut)]
pub struct Velocity(Vec3);

#[derive(Debug, Component, Clone, Copy, PartialEq, Default, Deref, DerefMut)]
pub struct PhysicalTranslation(Vec3);

#[derive(Debug, Component, Clone, Copy, PartialEq, Default, Deref, DerefMut)]
pub struct PreviousPhysicalTranslation(Vec3);
