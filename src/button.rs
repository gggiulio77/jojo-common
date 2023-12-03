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
