use bevy::prelude::*;
use serde::{Deserialize, Serialize};

/// Global game settings
#[derive(Debug, Resource, Clone, Serialize, Deserialize)]
pub struct GameSettings {
    pub graphics: GraphicsSettings,
    pub audio: AudioSettings,
    pub controls: ControlSettings,
}

impl Default for GameSettings {
    fn default() -> Self {
        Self {
            graphics: GraphicsSettings::default(),
            audio: AudioSettings::default(),
            controls: ControlSettings::default(),
        }
    }
}

/// Graphics configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GraphicsSettings {
    pub resolution: (u32, u32),
    pub vsync: bool,
    pub fps_limit: u32,
    pub render_distance: u32,
}

impl Default for GraphicsSettings {
    fn default() -> Self {
        Self {
            resolution: (1920, 1080),
            vsync: true,
            fps_limit: 60,
            render_distance: 16,
        }
    }
}

/// Audio configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AudioSettings {
    pub master_volume: f32,
    pub music_volume: f32,
    pub effects_volume: f32,
}

impl Default for AudioSettings {
    fn default() -> Self {
        Self {
            master_volume: 1.0,
            music_volume: 0.8,
            effects_volume: 0.9,
        }
    }
}

/// Control configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ControlSettings {
    pub mouse_sensitivity: f32,
    pub movement_speed: f32,
    pub key_bindings: KeyBindings,
}

impl Default for ControlSettings {
    fn default() -> Self {
        Self {
            mouse_sensitivity: 1.0,
            movement_speed: 5.0,
            key_bindings: KeyBindings::default(),
        }
    }
}

/// Keyboard bindings configuration
#[derive(Debug, Clone, Serialize)]
pub struct KeyBindings {
    pub move_forward: KeyCode,
    pub move_backward: KeyCode,
    pub move_left: KeyCode,
    pub move_right: KeyCode,
    pub jump: KeyCode,
    pub inventory: KeyCode,
}

impl Default for KeyBindings {
    fn default() -> Self {
        Self {
            move_forward: KeyCode::KeyW,
            move_backward: KeyCode::KeyS,
            move_left: KeyCode::KeyA,
            move_right: KeyCode::KeyD,
            jump: KeyCode::Space,
            inventory: KeyCode::KeyE,
        }
    }
}
