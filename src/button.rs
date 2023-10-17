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
pub enum Button {
    MouseButton(mouse::MouseButton),
    KeyboardButton(keyboard::KeyboardButton),
    CustomButton(CustomButton),
}

impl Default for Button {
    fn default() -> Self {
        Button::MouseButton(mouse::MouseButton::Left(mouse::MouseButtonState::Up))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test() {
        let key_button: Button = Button::CustomButton(CustomButton);
        let mouse_button: Button =
            Button::MouseButton(mouse::MouseButton::Left(mouse::MouseButtonState::Up));

        let buttons: Vec<Button> = vec![key_button, mouse_button];

        for button in buttons {
            match button {
                Button::MouseButton(mouse_button) => match mouse_button {
                    mouse::MouseButton::Left(_) => {
                        println!("Left click")
                    }
                    mouse::MouseButton::Middle(_) => {}
                    mouse::MouseButton::Right(_) => {}
                    mouse::MouseButton::Back(_) => {}
                    mouse::MouseButton::Forward(_) => {}
                    mouse::MouseButton::ScrollUp(_) => {}
                    mouse::MouseButton::ScrollDown(_) => {}
                    mouse::MouseButton::ScrollLeft(_) => {}
                    mouse::MouseButton::ScrollRight(_) => {}
                },
                Button::KeyboardButton(_keyboard_button) => {}
                Button::CustomButton(_custom_button) => {}
            }
        }
    }
}
