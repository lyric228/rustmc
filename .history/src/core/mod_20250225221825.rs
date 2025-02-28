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
fn load_settings<T, P>(path: P, format: &str) -> Result<T>
where
    T: DeserializeOwned,
    P: AsRef<Path>,
{
    let file = BFile::new(path)?;
    if !file.exists() {
        return Err(std::io::Error::new(
            std::io::ErrorKind::NotFound,
            "Settings file not found",
        ));
    }

    let content = file.read()?;
    
    match format.to_lowercase().as_str() {
        "json" => serde_json::from_str(&content).map_err(|e| std::io::Error::new(
            std::io::ErrorKind::InvalidData,
            format!("JSON parsing error: {}", e)
        )),
        "yaml" => serde_yaml::from_str(&content).map_err(|e| std::io::Error::new(
            std::io::ErrorKind::InvalidData,
            format!("YAML parsing error: {}", e)
        )),
        "toml" => toml::from_str(&content).map_err(|e| std::io::Error::new(
            std::io::ErrorKind::InvalidData,
            format!("TOML parsing error: {}", e)
        )),
        _ => Err(std::io::Error::new(
            std::io::ErrorKind::InvalidInput,
            "Unsupported format (supported: json, yaml, toml)"
        )),
    }
}
