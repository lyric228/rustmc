use serde::{Deserialize, Serialize};


#[derive(Serialize, Deserialize)]
pub struct GraphicsSettings {
    pub render_distance: u32,
    pub fov: f32,
    pub vsync: bool,
}

impl Default for GraphicsSettings {
    fn default() -> Self {
        Self {
            render_distance: 16,
            fov: 70.0,
            vsync: true,
        }
    }
}
