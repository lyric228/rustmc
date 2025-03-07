use bevy::prelude::*;

mod core;
mod player;
mod ui;
mod world;

const RESOLUTION: (f32, f32) = (
    1280.,
    720.,
);

fn main()
{
    App::new()
        .add_plugins((
            DefaultPlugins
                .set(WindowPlugin {
                    primary_window: Some(Window {
                        title: "RustMC".into(),
                        resolution: RESOLUTION.into(),
                        ..default()
                    }),
                    ..default()
                })
                .set(ImagePlugin::default_nearest()),
            core::CorePlugin,
            world::WorldPlugin,
            player::PlayerPlugin,
            // ui::UiPlugin,
        ))
        .run();
}
