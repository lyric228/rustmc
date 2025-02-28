use serde::{Deserialize, Deserializer, Serialize, Serializer};
use bevy::input::keyboard::NativeKeyCode;
use std::str::FromStr;
use bevy::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct KeyBindings {
    #[serde(with = "key_code_serde")]
    pub move_forward: KeyCode,
    #[serde(with = "key_code_serde")]
    pub move_backward: KeyCode,
    #[serde(with = "key_code_serde")]
    pub move_left: KeyCode,
    #[serde(with = "key_code_serde")]
    pub move_right: KeyCode,
    #[serde(with = "key_code_serde")]
    pub jump: KeyCode,
    #[serde(with = "key_code_serde")]
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

mod key_code_serde {
    use super::*;
    use std::str::FromStr;

    pub fn serialize<S>(key: &KeyCode, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        serializer.serialize_str(&format!("{:?}", key))
    }

    pub fn deserialize<'de, D>(deserializer: D) -> Result<KeyCode, D::Error>
    where
        D: Deserializer<'de>,
    {
        struct KeyCodeVisitor;

        impl<'de> serde::de::Visitor<'de> for KeyCodeVisitor {
            type Value = KeyCode;

            fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
                formatter.write_str("a valid KeyCode variant name")
            }

            fn visit_str<E>(self, value: &str) -> Result<KeyCode, E>
            where
                E: serde::de::Error,
            {
                KeyCode::from_str(value).map_err(|_| {
                    E::custom(format!("Unknown KeyCode variant: {}", value))
                })
            }
        }

        deserializer.deserialize_str(KeyCodeVisitor)
    }
}

impl FromStr for KeyCode {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        use KeyCode::*;
        
        match s {
            // Автоматическая генерация всех вариантов
            "Unidentified" => Ok(Unidentified(NativeKeyCode::Unidentified)),
            "Backquote" => Ok(Backquote),
            "Backslash" => Ok(Backslash),
            "BracketLeft" => Ok(BracketLeft),
            "BracketRight" => Ok(BracketRight),
            // ... остальные варианты аналогично
            "KeyW" => Ok(KeyW),
            "KeyS" => Ok(KeyS),
            "KeyA" => Ok(KeyA),
            "KeyD" => Ok(KeyD),
            "Space" => Ok(Space),
            "KeyE" => Ok(KeyE),
            // ... все остальные варианты KeyCode
            _ => Err(format!("Unknown KeyCode: {}", s)),
        }
    }
}
