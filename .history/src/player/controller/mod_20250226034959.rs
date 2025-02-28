mod keyboard;
mod mouse;

pub use keyboard::*;
pub use mouse::*;
use bevy::prelude::*;
use crate::player::types::*;

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

pub fn spawn_text(mut commands: Commands) {
    commands
        .spawn(Node {
            position_type: PositionType::Absolute,
            bottom: Val::Px(12.0),
            left: Val::Px(12.0),
            ..default()
        })
        .with_child((
            Text::new("Move the player with WASD"),
            TextFont {
                font_size: 25.0,
                ..default()
            },
        ));
}


pub fn advance_physics(
    fixed_time: Res<Time<Fixed>>,
    mut query: Query<(
        &mut PhysicalTranslation,
        &mut PreviousPhysicalTranslation,
        &mut AccumulatedInput,
        &Velocity,
    )>,
) {
    for (
        mut current_physical_translation,
        mut previous_physical_translation,
        mut input,
        velocity,
    ) in query.iter_mut()
    {
        previous_physical_translation = current_physical_translation.;
        current_physical_translation += velocity * fixed_time.delta_secs();

        // Reset the input accumulator, as we are currently consuming all input that happened since the last fixed timestep.
        input.0 = Vec2::ZERO;
    }
}
