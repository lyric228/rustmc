use crate::player::types::*;

use bevy::prelude::*;
use super::Player;


pub fn spawn_camera(mut commands: Commands) {
    // commands.spawn((
    //     Camera3d::,
    //     Player {
    //         movement_speed: 5.0,
    //         mouse_sensitivity: 0.5,
    //     }
    // ));
}



pub fn spawn_player(mut commands: Commands, asset_server: Res<AssetServer>) {
    commands.spawn(Camera2d);
    commands.spawn((
        Name::new("Player"),
        Sprite::from_image(asset_server.load("branding/icon.png")),
        Transform::from_scale(Vec3::splat(0.3)),
        AccumulatedInput::default(),
        Velocity::default(),
        PhysicalTranslation::default(),
        PreviousPhysicalTranslation::default(),
    ));
}
