use bevy::input::keyboard::NativeKeyCode;
use bevy::prelude::*;
use serde::{Deserialize, Deserializer, Serialize, Serializer};
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

pub mod key_code_serde {
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
pub fn parse_keycode(s: &str) -> Result<KeyCode, String> {
    use KeyCode::*;

    match s {
        // Основные клавиши
        "Unidentified" => Ok(Unidentified(NativeKeyCode::Unidentified)),
        "Backquote" => Ok(Backquote),
        "Backslash" => Ok(Backslash),
        "BracketLeft" => Ok(BracketLeft),
        "BracketRight" => Ok(BracketRight),
        "Comma" => Ok(Comma),
        "Digit0" => Ok(Digit0),
        "Digit1" => Ok(Digit1),
        "Digit2" => Ok(Digit2),
        "Digit3" => Ok(Digit3),
        "Digit4" => Ok(Digit4),
        "Digit5" => Ok(Digit5),
        "Digit6" => Ok(Digit6),
        "Digit7" => Ok(Digit7),
        "Digit8" => Ok(Digit8),
        "Digit9" => Ok(Digit9),
        "Equal" => Ok(Equal),
        "IntlBackslash" => Ok(IntlBackslash),
        "IntlRo" => Ok(IntlRo),
        "IntlYen" => Ok(IntlYen),

        // Буквенные клавиши
        "KeyA" => Ok(KeyA),
        "KeyB" => Ok(KeyB),
        "KeyC" => Ok(KeyC),
        "KeyD" => Ok(KeyD),
        "KeyE" => Ok(KeyE),
        "KeyF" => Ok(KeyF),
        "KeyG" => Ok(KeyG),
        "KeyH" => Ok(KeyH),
        "KeyI" => Ok(KeyI),
        "KeyJ" => Ok(KeyJ),
        "KeyK" => Ok(KeyK),
        "KeyL" => Ok(KeyL),
        "KeyM" => Ok(KeyM),
        "KeyN" => Ok(KeyN),
        "KeyO" => Ok(KeyO),
        "KeyP" => Ok(KeyP),
        "KeyQ" => Ok(KeyQ),
        "KeyR" => Ok(KeyR),
        "KeyS" => Ok(KeyS),
        "KeyT" => Ok(KeyT),
        "KeyU" => Ok(KeyU),
        "KeyV" => Ok(KeyV),
        "KeyW" => Ok(KeyW),
        "KeyX" => Ok(KeyX),
        "KeyY" => Ok(KeyY),
        "KeyZ" => Ok(KeyZ),

        // Символы и модификаторы
        "Minus" => Ok(Minus),
        "Period" => Ok(Period),
        "Quote" => Ok(Quote),
        "Semicolon" => Ok(Semicolon),
        "Slash" => Ok(Slash),
        "AltLeft" => Ok(AltLeft),
        "AltRight" => Ok(AltRight),
        "Backspace" => Ok(Backspace),
        "CapsLock" => Ok(CapsLock),
        "ContextMenu" => Ok(ContextMenu),
        "ControlLeft" => Ok(ControlLeft),
        "ControlRight" => Ok(ControlRight),
        "Enter" => Ok(Enter),
        "SuperLeft" => Ok(SuperLeft),
        "SuperRight" => Ok(SuperRight),
        "ShiftLeft" => Ok(ShiftLeft),
        "ShiftRight" => Ok(ShiftRight),
        "Space" => Ok(Space),
        "Tab" => Ok(Tab),

        // Специальные клавиши
        "Convert" => Ok(Convert),
        "KanaMode" => Ok(KanaMode),
        "Lang1" => Ok(Lang1),
        "Lang2" => Ok(Lang2),
        "Lang3" => Ok(Lang3),
        "Lang4" => Ok(Lang4),
        "Lang5" => Ok(Lang5),
        "NonConvert" => Ok(NonConvert),

        // Навигационные клавиши
        "Delete" => Ok(Delete),
        "End" => Ok(End),
        "Help" => Ok(Help),
        "Home" => Ok(Home),
        "Insert" => Ok(Insert),
        "PageDown" => Ok(PageDown),
        "PageUp" => Ok(PageUp),

        // Стрелки
        "ArrowDown" => Ok(ArrowDown),
        "ArrowLeft" => Ok(ArrowLeft),
        "ArrowRight" => Ok(ArrowRight),
        "ArrowUp" => Ok(ArrowUp),

        // NumLock и Numpad
        "NumLock" => Ok(NumLock),
        "Numpad0" => Ok(Numpad0),
        "Numpad1" => Ok(Numpad1),
        "Numpad2" => Ok(Numpad2),
        "Numpad3" => Ok(Numpad3),
        "Numpad4" => Ok(Numpad4),
        "Numpad5" => Ok(Numpad5),
        "Numpad6" => Ok(Numpad6),
        "Numpad7" => Ok(Numpad7),
        "Numpad8" => Ok(Numpad8),
        "Numpad9" => Ok(Numpad9),
        "NumpadAdd" => Ok(NumpadAdd),
        "NumpadBackspace" => Ok(NumpadBackspace),
        "NumpadClear" => Ok(NumpadClear),
        "NumpadClearEntry" => Ok(NumpadClearEntry),
        "NumpadComma" => Ok(NumpadComma),
        "NumpadDecimal" => Ok(NumpadDecimal),
        "NumpadDivide" => Ok(NumpadDivide),
        "NumpadEnter" => Ok(NumpadEnter),
        "NumpadEqual" => Ok(NumpadEqual),
        "NumpadHash" => Ok(NumpadHash),
        "NumpadMemoryAdd" => Ok(NumpadMemoryAdd),
        "NumpadMemoryClear" => Ok(NumpadMemoryClear),
        "NumpadMemoryRecall" => Ok(NumpadMemoryRecall),
        "NumpadMemoryStore" => Ok(NumpadMemoryStore),
        "NumpadMemorySubtract" => Ok(NumpadMemorySubtract),
        "NumpadMultiply" => Ok(NumpadMultiply),
        "NumpadParenLeft" => Ok(NumpadParenLeft),
        "NumpadParenRight" => Ok(NumpadParenRight),
        "NumpadStar" => Ok(NumpadStar),
        "NumpadSubtract" => Ok(NumpadSubtract),

        // Функциональные клавиши
        "Escape" => Ok(Escape),
        "Fn" => Ok(Fn),
        "FnLock" => Ok(FnLock),
        "PrintScreen" => Ok(PrintScreen),
        "ScrollLock" => Ok(ScrollLock),
        "Pause" => Ok(Pause),

        // Мультимедийные клавиши
        "BrowserBack" => Ok(BrowserBack),
        "BrowserFavorites" => Ok(BrowserFavorites),
        "BrowserForward" => Ok(BrowserForward),
        "BrowserHome" => Ok(BrowserHome),
        "BrowserRefresh" => Ok(BrowserRefresh),
        "BrowserSearch" => Ok(BrowserSearch),
        "BrowserStop" => Ok(BrowserStop),
        "Eject" => Ok(Eject),
        "LaunchApp1" => Ok(LaunchApp1),
        "LaunchApp2" => Ok(LaunchApp2),
        "LaunchMail" => Ok(LaunchMail),
        "MediaPlayPause" => Ok(MediaPlayPause),
        "MediaSelect" => Ok(MediaSelect),
        "MediaStop" => Ok(MediaStop),
        "MediaTrackNext" => Ok(MediaTrackNext),
        "MediaTrackPrevious" => Ok(MediaTrackPrevious),
        "Power" => Ok(Power),
        "Sleep" => Ok(Sleep),
        "AudioVolumeDown" => Ok(AudioVolumeDown),
        "AudioVolumeMute" => Ok(AudioVolumeMute),
        "AudioVolumeUp" => Ok(AudioVolumeUp),
        "WakeUp" => Ok(WakeUp),

        // Легаси и специальные
        "Meta" => Ok(Meta),
        "Hyper" => Ok(Hyper),
        "Turbo" => Ok(Turbo),
        "Abort" => Ok(Abort),
        "Resume" => Ok(Resume),
        "Suspend" => Ok(Suspend),
        "Again" => Ok(Again),
        "Copy" => Ok(Copy),
        "Cut" => Ok(Cut),
        "Find" => Ok(Find),
        "Open" => Ok(Open),
        "Paste" => Ok(Paste),
        "Props" => Ok(Props),
        "Select" => Ok(Select),
        "Undo" => Ok(Undo),
        "Hiragana" => Ok(Hiragana),
        "Katakana" => Ok(Katakana),

        // Функциональные клавиши F1-F35
        "F1" => Ok(F1),
        "F2" => Ok(F2),
        "F3" => Ok(F3),
        "F4" => Ok(F4),
        "F5" => Ok(F5),
        "F6" => Ok(F6),
        "F7" => Ok(F7),
        "F8" => Ok(F8),
        "F9" => Ok(F9),
        "F10" => Ok(F10),
        "F11" => Ok(F11),
        "F12" => Ok(F12),
        "F13" => Ok(F13),
        "F14" => Ok(F14),
        "F15" => Ok(F15),
        "F16" => Ok(F16),
        "F17" => Ok(F17),
        "F18" => Ok(F18),
        "F19" => Ok(F19),
        "F20" => Ok(F20),
        "F21" => Ok(F21),
        "F22" => Ok(F22),
        "F23" => Ok(F23),
        "F24" => Ok(F24),
        "F25" => Ok(F25),
        "F26" => Ok(F26),
        "F27" => Ok(F27),
        "F28" => Ok(F28),
        "F29" => Ok(F29),
        "F30" => Ok(F30),
        "F31" => Ok(F31),
        "F32" => Ok(F32),
        "F33" => Ok(F33),
        "F34" => Ok(F34),
        "F35" => Ok(F35),

        // Обработка неизвестных значений
        _ => Err(format!("Unknown KeyCode: {}", s)),
    }
}
