use crate::{keyboard, mouse};
use serde::{Deserialize, Serialize};

// TODO: find about what can we do, Actions like open chrome bound to a button, or a Key of the SO (like ctrl, alt, etc) or a sequence of things
#[derive(Debug, Clone, Deserialize, Serialize, Eq, PartialEq)]
pub struct CustomButton;

#[derive(Debug, Clone, Deserialize, Serialize, Eq, PartialEq)]
#[serde(rename_all = "snake_case")]
pub enum ButtonAction {
    MouseButton(mouse::MouseButton, mouse::MouseButtonState),
    KeyboardButton(keyboard::KeyboardButton),
    CustomButton(CustomButton),
}

pub type ButtonId = uuid::Uuid;

#[derive(Debug, Clone, Deserialize, Serialize, Eq, PartialEq)]
#[serde(rename_all = "snake_case")]
pub enum ButtonMode {
    Hold,
    Click,
}

// This is holds the mcu button info, is used to map a button to an action
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

    #[test]
    fn test() {
        let key_button = ButtonAction::CustomButton(CustomButton);
        let mouse_button =
            ButtonAction::MouseButton(mouse::MouseButton::Left, mouse::MouseButtonState::Up);

        let actions: Vec<ButtonAction> = vec![key_button, mouse_button];

        for button in actions {
            match button {
                ButtonAction::MouseButton(mouse_button, state) => match mouse_button {
                    mouse::MouseButton::Left => {
                        println!("Left click")
                    }
                    mouse::MouseButton::Middle => {}
                    mouse::MouseButton::Right => {}
                    mouse::MouseButton::Back => {}
                    mouse::MouseButton::Forward => {}
                    mouse::MouseButton::ScrollUp => {}
                    mouse::MouseButton::ScrollDown => {}
                    mouse::MouseButton::ScrollLeft => {}
                    mouse::MouseButton::ScrollRight => {}
                },
                ButtonAction::KeyboardButton(_keyboard_button) => {}
                ButtonAction::CustomButton(_custom_button) => {}
            }
        }
    }
}
