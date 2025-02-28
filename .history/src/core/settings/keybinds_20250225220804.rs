use serde::{Deserialize, Deserializer, Serialize, Serializer};
use bevy::input::keyboard::NativeKeyCode;
use bevy::prelude::*;
use std::fmt;

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
    pub sprint: KeyCode,
    #[serde(with = "key_code_serde")]
    pub sneak: KeyCode,
    #[serde(with = "key_code_serde")]
    pub inventory: KeyCode,
    #[serde(with = "key_code_serde")]
    pub drop: KeyCode,
}

impl Default for KeyBindings {
    fn default() -> Self {
        Self {
            move_forward: KeyCode::KeyW,
            move_backward: KeyCode::KeyS,
            move_left: KeyCode::KeyA,
            move_right: KeyCode::KeyD,
            sprint: KeyCode::ShiftLeft,
            sneak: KeyCode::ControlLeft,
            jump: KeyCode::Space,
            inventory: KeyCode::Tab,
            drop: KeyCode::KeyQ,
        }
    }
}

mod key_code_serde {
    use super::*;
    
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
            type Value = String;

            fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
                formatter.write_str("a valid KeyCode variant name")
            }

            fn visit_str<E>(self, value: &str) -> Result<String, E>
            where
                E: serde::de::Error,
            {
                Ok(value.to_owned())
            }
        }
        
        let s = deserializer.deserialize_str(KeyCodeVisitor)?;
        parse_keycode(&s).map_err(serde::de::Error::custom)
    }
}

// Вместо реализации FromStr для KeyCode создаем свою функцию парсинга
fn parse_keycode(s: &str) -> Result<KeyCode, String> {
    use KeyCode::*;
    
    match s {
        "Unidentified" => Ok(Unidentified(NativeKeyCode::Unidentified)),
        "Backquote" => Ok(Backquote),
        "Backslash" => Ok(Backslash),
        "BracketLeft" => Ok(BracketLeft),
        "BracketRight" => Ok(BracketRight),
        "KeyW" => Ok(KeyW),
        "KeyS" => Ok(KeyS),
        "KeyA" => Ok(KeyA),
        "KeyD" => Ok(KeyD),
        "Space" => Ok(Space),
        "KeyE" => Ok(KeyE),
        // Добавьте остальные варианты...
        _ => Err(format!("Unknown KeyCode: {}", s)),
    }
}
