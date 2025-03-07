mod controller;
mod types;

use bevy::prelude::*;
pub use types::*;

pub struct PlayerPlugin;

impl Plugin for PlayerPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, (spawn_player, controller::mouse::grab_mouse))
            .add_systems(
                Update,
                (
                    controller::physics::handle_input,
                    controller::mouse::mouse_move_system,
                    controller::mouse::mouse_click_system,
                    controller::physics::apply_friction,
                    controller::keyboard::controller::kb_input_events,
                ),
            )
            .add_systems(FixedUpdate, controller::physics::advance_physics)
            .add_systems(
                PostUpdate,
                controller::physics::interpolate_rendered_transform,
            );
    }
}

fn spawn_player(mut commands: Commands) {
    commands.spawn((
        Player,
        AccumulatedInput::default(),
        Velocity::default(),
        PhysicalTranslation(Vec3::ZERO),
        PreviousPhysicalTranslation(Vec3::ZERO),
        PlayerLook::default(), // Add the new component
        Transform::from_xyz(0.0, 0.0, 0.0),
    ));
}
