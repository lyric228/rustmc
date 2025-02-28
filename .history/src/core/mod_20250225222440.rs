mod game_state;
mod settings;

pub use game_state::*;
pub use settings::*;

use sysx::io::fs::BFile;
use bevy::prelude::*;
use sysx::Result;
use serde::{Serialize, de::DeserializeOwned};
use std::path::Path;

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
pub fn load_settings<T, P>(path: P) -> sysx::Result<T>
where
    T: Serialize + DeserializeOwned + Default,
    P: AsRef<Path>,
{
    let file = BFile::new(path.as_ref())?;
    
    // Создаем файл с настройками по умолчанию если не существует
    if !file.exists() {
        let default_settings = T::default();
        let toml_str = toml::to_string_pretty(&default_settings)
            .map_err(|e| SysxError::AnyhowError(e.into()))?;
        
        file.write(&toml_str)?;
    }

    // Читаем и парсим содержимое файла
    let content = file.read()?;
    let settings = toml::from_str(&content)
        .map_err(|e| SysxError::AnyhowError(e.into()))?;

    Ok(settings)
}
