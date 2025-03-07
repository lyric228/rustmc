pub mod keyboard;
pub mod mouse;
pub mod physics;

pub use keyboard::controller::*;
pub use mouse::*;
pub use physics::*;

use crate::player::types::*;
use bevy::prelude::*;

pub fn spawn_player(mut commands: Commands, asset_server: Res<AssetServer>) {
    let player = commands
        .spawn((
            Player,
            Name::new("Player"),
            Sprite::from_image(asset_server.load("skins/default.png")),
            Transform::from_scale(Vec3::from_array([1.0, 2.0, 1.0])),
            AccumulatedInput::default(),
            Velocity::default(),
            PhysicalTranslation::default(),
            PreviousPhysicalTranslation::default(),
            PlayerLook::default(), // Добавляем компонент для управления поворотом головы
        ))
        .id();

    commands.entity(player).with_children(|parent| {
        parent.spawn((
            Camera3d::default(),
            Transform::from_xyz(0.0, 2.0, 2.0).looking_at(Vec3::new(0.0, 0.0, -2.0), Vec3::Y),
        ));
    });
}

pub fn spawn_light(mut commands: Commands) {
    commands.spawn((
        DirectionalLight {
            shadows_enabled: true,
            illuminance: 10000.0,
            ..default()
        },
        Transform::from_xyz(4.0, 8.0, 4.0).looking_at(Vec3::ZERO, Vec3::Y),
    ));
}

pub fn spawn_text<'a>(
    mut commands: Commands,
    node: impl Into<Node>,
    text_bundle: impl Bundle,
) -> Entity {
    commands
        .spawn(node.into())
        .with_children(|parent| {
            parent.spawn(text_bundle);
        })
        .id()
}

pub fn spawn_text_default(
    commands: Commands,
    asset_server: Res<AssetServer>,
    size: f32,
    text: impl Into<String>,
) -> Entity {
    spawn_text(
        commands,
        Node {
            width: Val::Percent(100.0),
            height: Val::Percent(100.0),
            position_type: PositionType::Absolute,
            justify_content: JustifyContent::FlexStart,
            align_items: AlignItems::FlexEnd,
            padding: UiRect::all(Val::Px(10.0)),
            ..default()
        },
        (
            Text::new(text),
            TextFont {
                font: asset_server.load("fonts/noto_serif.ttf"),
                font_size: size,
                ..Default::default()
            },
        ),
    )
}
