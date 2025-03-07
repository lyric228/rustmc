use bevy::prelude::*;
use serde::{Serialize, de::DeserializeOwned};
use std::path::Path;
use sysx::io::fs::BFile;
pub mod settings;
pub mod states;
pub use settings::GameSettings;
pub use sysx::Result;
pub use sysx::SysxError;
pub struct CorePlugin;

impl Plugin for CorePlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<GameSettings>();
    }
}

pub fn load_settings<T, P>(path: P) -> Result<T>
where
    T: Serialize + DeserializeOwned + Default,
    P: AsRef<Path>,
{
    let file = BFile::new(path.as_ref())?;

    // Create default settings file if it doesn't exist
    if !file.exists() {
        let default_settings = T::default();
        let toml_str = toml::to_string_pretty(&default_settings)
            .map_err(|e| SysxError::AnyhowError(e.into()))?;

        file.write(&toml_str)?;
    }

    // Read and parse file content
    let content = file.read()?;
    let settings = toml::from_str(&content).map_err(|e| SysxError::AnyhowError(e.into()))?;

    Ok(settings)
}
