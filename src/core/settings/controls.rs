use serde::{Deserialize, Serialize};


#[derive(Serialize, Deserialize)]
pub struct ControlsSettings {
    pub mouse_sensitivity: f32,
    pub invert_y: bool,
}

impl Default for ControlsSettings {
    fn default() -> Self {
        Self {
            mouse_sensitivity: 0.002,
            invert_y: false,
        }
    }
}

