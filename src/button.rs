use crate::{keyboard, mouse};
use serde::{Deserialize, Serialize};

// TODO: find about what can we do, Actions like open chrome bound to a button, or a Key of the SO (like ctrl, alt, etc) or a sequence of things
#[derive(Debug, Clone, Deserialize, Serialize, Eq, PartialEq)]
pub struct CustomButton;

#[derive(Debug, Clone, Deserialize, Serialize, Eq, PartialEq)]
#[serde(rename_all = "snake_case")]
pub enum ButtonRead {
    MouseButton(mouse::MouseButton),
    KeyboardButton(keyboard::KeyboardButton),
    CustomButton(CustomButton),
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test() {
        let key_button: ButtonRead = ButtonRead::CustomButton(CustomButton);
        let mouse_button: ButtonRead =
            ButtonRead::MouseButton(mouse::MouseButton::Left(mouse::MouseButtonState::Up));

        let buttons: Vec<ButtonRead> = vec![key_button, mouse_button];

        for button in buttons {
            match button {
                ButtonRead::MouseButton(mouse_button) => match mouse_button {
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
                ButtonRead::KeyboardButton(_keyboard_button) => {}
                ButtonRead::CustomButton(_custom_button) => {}
            }
        }
    }
}
