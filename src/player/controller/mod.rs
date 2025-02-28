mod keyboard_controller; // Updated to reflect new file structure
mod mouse_controller; // Updated to reflect new file structure

pub use keyboard_controller::*; // Updated to reflect new file structure
pub use mouse_controller::*; // Updated to reflect new file structure
use bevy::prelude::*;
use crate::player::types::*;

pub fn spawn_player(mut commands: Commands, asset_server: Res<AssetServer>) {
    commands.spawn(Camera2d);
    commands.spawn((
        Name::new("Player"),
        Sprite::from_image(asset_server.load("textures/skin/skin.png")),
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


pub fn handle_input(
    keyboard_input: Res<ButtonInput<KeyCode>>,
    mut query: Query<(&mut AccumulatedInput, &mut Velocity)>,
) {
    /// Since Bevy's default 2D camera setup is scaled such that
    /// one unit is one pixel, you can think of this as
    /// "How many pixels per second should the player move?"
    const SPEED: f32 = 210.0;
    for (mut input, mut velocity) in query.iter_mut() {
        if keyboard_input.pressed(KeyCode::KeyW) {
            input.y += 1.0;
        }
        if keyboard_input.pressed(KeyCode::KeyS) {
            input.y -= 1.0;
        }
        if keyboard_input.pressed(KeyCode::KeyA) {
            input.x -= 1.0;
        }
        if keyboard_input.pressed(KeyCode::KeyD) {
            input.x += 1.0;
        }

        // Need to normalize and scale because otherwise
        // diagonal movement would be faster than horizontal or vertical movement.
        // This effectively averages the accumulated input.
        // HERE VELOCITY CHANGE
        velocity.x = input.x.clamp(-1.0, 1.0) * SPEED;
        velocity.y = input.y.clamp(-1.0, 1.0) * SPEED;
    }
}
