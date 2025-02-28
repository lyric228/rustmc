mod game_state;
mod settings;

use bevy::prelude::*;
pub use game_state::GameState;
pub use settings::GameSettings;

pub struct CorePlugin;

impl Plugin for CorePlugin {
    fn build(&self, app: &mut App) {
        app
            .init_state::<GameState>()
            .init_resource::<GameSettings>();
    }
}
