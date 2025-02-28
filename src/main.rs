use bevy::prelude::*;

mod core;
mod world;
mod player;
mod ui;

fn main() {
    App::new()
        .add_plugins((
            DefaultPlugins.set(WindowPlugin {
                primary_window: Some(Window {
                    title: "RustMC".into(),
                    resolution: (1280., 720.).into(),
                    ..default()
                }),
                ..default()
            }),
            core::CorePlugin,
            world::WorldPlugin,
            player::PlayerPlugin,
            // ui::UiPlugin,
        ))
        .run();
}
