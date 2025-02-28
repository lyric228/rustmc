mod game_state;
mod settings;

pub use game_state::*;
pub use settings::*;

use bevy::prelude::*;

/// Core plugin handling game state and settings
pub struct CorePlugin;

impl Plugin for CorePlugin {
    fn build(&self, app: &mut App) {
        app
            .init_state::<GameState>()
            .init_resource::<GameSettings>()
            .add_systems(Startup, load_settings)
            .configure_sets(
                Update,
                (
                    GameSystemSet::MainMenu.run_if(in_state(GameState::MainMenu)),
                    GameSystemSet::InGame.run_if(in_state(GameState::Gameplay)),
                )
            );
    }
}

/// System for loading settings from file
fn load_settings(mut settings: ResMut<GameSettings>) {
    // Здесь можно добавить загрузку настроек из файла
    // Например, используя sysx::fs
    // *settings = load_from_file().unwrap_or_default();
}
