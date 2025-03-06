pub mod camera;
pub mod controller;
pub mod types;

use crate::player::controller::physics::*;
use crate::player::controller::*;
use crate::player::types::*;
use bevy::prelude::*;

pub struct PlayerPlugin;

impl Plugin for PlayerPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(
            Startup,
            (
                |commands: Commands, asset_server: Res<AssetServer>| {
                    spawn_text_default(commands, asset_server, 25.0, "текст");
                },
                spawn_player,
                spawn_light,
            ),
        )
        // Системы, которые должны работать каждый кадр
        .add_systems(
            Update,
            (
                controller::mouse_click_system,
                controller::mouse_move_system,
                controller::grab_mouse,
                controller::kb_input_events,
            ),
        )
        // Системы для фиксированного временного шага (физика)
        .add_systems(FixedUpdate, (advance_physics, apply_friction))
        // Системы для обработки ввода и интерполяции
        .add_systems(
            RunFixedMainLoop,
            (
                handle_input.in_set(RunFixedMainLoopSystem::BeforeFixedMainLoop),
                interpolate_rendered_transform.in_set(RunFixedMainLoopSystem::AfterFixedMainLoop),
            ),
        );
    }
}
