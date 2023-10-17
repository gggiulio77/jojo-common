use crate::{keyboard, mouse};
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Deserialize, Serialize, PartialEq)]
pub struct ButtonRead {
    kind: Button,
}

impl ButtonRead {
    pub fn new(kind: Button) -> Self {
        ButtonRead { kind }
    }

    pub fn kind(&self) -> &Button {
        &self.kind
    }
}

// TODO: find about what can we do, Actions like open chrome bound to a button, or a Key of the SO (like ctrl, alt, etc) or a sequence of things
#[derive(Debug, Clone, Deserialize, Serialize, Eq, PartialEq)]
pub struct CustomButton;

#[derive(Debug, Clone, Deserialize, Serialize, Eq, PartialEq)]
pub enum MouseButton {
    Left,
    Middle,
    Right,
    Back,
    Forward,

    ScrollUp,
    ScrollDown,
    ScrollLeft,
    ScrollRight,
}

#[derive(Debug, Clone, Deserialize, Serialize, Eq, PartialEq)]
pub enum Button {
    MouseButton(MouseButton),
    CustomButton(CustomButton),
}

impl Default for Button {
    fn default() -> Self {
        Button::MouseButton(MouseButton::Left)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test() {
        let key_button: Button = Button::CustomButton(CustomButton);
        let mouse_button: Button = Button::MouseButton(MouseButton::Left);

        let buttons: Vec<Button> = vec![key_button, mouse_button];

        for button in buttons {
            match button {
                Button::MouseButton(mouse_button) => match mouse_button {
                    MouseButton::Left => {
                        println!("Left click")
                    }
                    MouseButton::Middle => {}
                    MouseButton::Right => {}
                    MouseButton::Back => {}
                    MouseButton::Forward => {}
                    MouseButton::ScrollUp => {}
                    MouseButton::ScrollDown => {}
                    MouseButton::ScrollLeft => {}
                    MouseButton::ScrollRight => {}
                },
                Button::CustomButton(keyboard_button) => {}
            }
        }
    }
}
