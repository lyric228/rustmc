use bevy::prelude::*;

#[derive(Component)]
pub struct Player;

#[derive(Component)]
pub struct WorldModelCamera;

#[derive(Component, Default)]
pub struct AccumulatedInput {
    pub x: f32,
    pub y: f32,
}

#[derive(Component, Default)]
pub struct Velocity(pub Vec3);

#[derive(Component, Default)]
pub struct PhysicalTranslation(pub Vec3);

#[derive(Component, Default)]
pub struct PreviousPhysicalTranslation(pub Vec3);

#[derive(Component, Default)]
pub struct CameraSensitivity {
    pub value: f32,
}

// Новый компонент для поворота головы
#[derive(Component, Default, Debug)]
pub struct PlayerLook {
    pub yaw: f32,   // Поворот по горизонтали (в радианах)
    pub pitch: f32, // Наклон по вертикали (в радианах)
}
