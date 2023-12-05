use crate::command::CustomCommand;
use crate::{gamepad, keyboard, mouse};
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Deserialize, Serialize, Eq, PartialEq)]
#[serde(rename_all = "snake_case")]
pub enum ButtonAction {
    MouseButton(mouse::MouseButton, mouse::MouseButtonState),
    KeyboardButton(keyboard::KeyboardButton),
    GamepadButton(gamepad::GamepadButton, gamepad::GamepadButtonState),
    CustomButton(CustomCommand),
}

pub type ButtonId = uuid::Uuid;

#[derive(Debug, Clone, Deserialize, Serialize, Eq, PartialEq)]
#[serde(rename_all = "snake_case")]
pub enum ButtonMode {
    Hold,
    Click,
}

// This holds the mcu button info, is used to map a button to an action
// This is immutable
#[derive(Debug, Clone, Deserialize, Serialize, Eq, PartialEq)]
#[serde(rename_all = "snake_case")]
pub struct Button {
    id: ButtonId,
    name: String,
    mode: ButtonMode,
}

impl Button {
    pub fn new(id: ButtonId, name: String, mode: ButtonMode) -> Self {
        Button { id, name, mode }
    }

    pub fn id(&self) -> ButtonId {
        self.id
    }
    pub fn name(&self) -> &str {
        &self.name
    }
    pub fn mode(&self) -> &ButtonMode {
        &self.mode
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::keyboard::{Key, KeyboardButton};
    use crate::message::ServerMessage;
    use std::collections::HashMap;
    use uuid::uuid;

    #[test]
    fn test_serialize() {
        let button_actions: Vec<ButtonAction> = vec![
            ButtonAction::CustomButton(CustomCommand::Binary(
                "C:\\Users\\gggiu\\AppData\\Roaming\\Spotify\\Spotify.exe".to_string(),
            )),
            ButtonAction::KeyboardButton(KeyboardButton::Key(Key::A)),
            ButtonAction::KeyboardButton(KeyboardButton::Key(Key::A)),
        ];
        let message = crate::message::ServerMessage::UpdateDevice(
            uuid!("58c79037-d101-476d-bcbe-1503e9011261"),
            HashMap::from([(
                uuid!("0ce7ecdb-4dcc-46f5-804c-65a39d2277a0"),
                button_actions,
            )]),
        );
        let serialized = bincode::serialize(&message).unwrap();
        let serialized_text = serde_json::to_string(&message).unwrap();
        let deserialized =
            bincode::deserialize::<crate::message::ServerMessage>(&serialized).unwrap();
        let deserialized_text: ServerMessage = serde_json::from_str(&serialized_text).unwrap();

        println!("[serialized]: {:?}", serialized);

        assert_eq!(deserialized, message);
        assert_eq!(deserialized_text, message);
    }
}
