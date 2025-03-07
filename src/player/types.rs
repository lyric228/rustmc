use bevy::prelude::*;

#[derive(Component, Default, Debug)]
pub struct Player;

#[derive(Component, Debug, Default)]
pub struct AccumulatedInput {
    pub x: f32,
    pub y: f32,
}

#[derive(Component, Debug, Default)]
pub struct Velocity(pub Vec3);

#[derive(Component, Debug, Default)]
pub struct PhysicalTranslation(pub Vec3);

#[derive(Component, Debug, Default)]
pub struct PreviousPhysicalTranslation(pub Vec3);

#[derive(Component, Default, Debug)]
pub struct PlayerLook {
    pub yaw: f32,   // Horizontal rotation (in radians)
    pub pitch: f32, // Vertical rotation (in radians)
}
