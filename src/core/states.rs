use bevy::prelude::*;

/// Main game states
#[derive(Debug, Clone, Copy, Default, Eq, PartialEq, Hash, States)]
pub enum GameState {
    #[default]
    MainMenu,
    Gameplay,
    Paused,
    Settings,
    GameOver,
}

/// System sets for state-specific logic
#[derive(Debug, Hash, PartialEq, Eq, Clone, SystemSet)]
pub enum GameSystemSet {
    MainMenu,
    InGame,
}
