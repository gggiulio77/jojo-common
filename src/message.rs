use serde::{Deserialize, Serialize};
use std::collections::HashMap;

use crate::{button, device, mouse};

#[derive(Serialize, Deserialize, PartialEq, Debug, Clone)]
#[serde(rename_all = "snake_case")]
pub struct Reads {
    mouse_read: Option<mouse::MouseRead>,
    button_actions: Option<Vec<button::ButtonAction>>,
}

impl Reads {
    pub fn new(
        mouse_read: Option<mouse::MouseRead>,
        button_actions: Option<Vec<button::ButtonAction>>,
    ) -> Self {
        Reads {
            mouse_read,
            button_actions,
        }
    }

    pub fn mouse_read(&self) -> &Option<mouse::MouseRead> {
        &self.mouse_read
    }

    pub fn button_actions(&self) -> &Option<Vec<button::ButtonAction>> {
        &self.button_actions
    }
}

#[derive(Serialize, Deserialize, PartialEq, Debug)]
#[serde(rename_all = "snake_case")]
pub enum ClientMessage {
    Reads(Vec<Reads>),
    Device(device::Device),
}

#[cfg(test)]
mod tests {
    use crate::{keyboard, keyboard::KeyboardButton};
    use std::collections::HashMap;

    use super::*;
    use crate::button::{Button, ButtonAction, ButtonMode};
    use crate::keyboard::Key;
    use crate::mouse::{MouseButton, MouseButtonState};
    use uuid::uuid;

    #[test]
    fn test_serialize_message() {
        let device_msg = r#"{"device": {"id": "340917e8-87a9-455c-9645-d08eb99162f9","name": "asd","mouse_config": null,"buttons": [{ "id": "340917e8-87a9-455c-9645-d08eb99162f1", "name": "button_0", "mode": "hold" }], "actions_map": { "340917e8-87a9-455c-9645-d08eb99162f1": [{ "keyboard_button": { "key": "A" } }] } }}"#;
        let reads_msg = r#"{"reads": [{"mouse_read": {"x_read": 100, "y_read": 100}, "button_actions": [{"mouse_button": ["left", "up"]}, {"keyboard_button": {"key": "A"}}, {"keyboard_button": {"sequence": "Hello World!❤️"}}, {"keyboard_button": {"sequence_dsl": "{CTRL+}a{CTRL-}"}}]}]}"#;
        let id = uuid!("340917e8-87a9-455c-9645-d08eb99162f9");

        let device_result: ClientMessage = serde_json::from_str(device_msg).unwrap();
        let reads_result: ClientMessage = serde_json::from_str(reads_msg).unwrap();

        assert_eq!(
            device_result,
            ClientMessage::Device(device::Device::new(
                id,
                format!("asd"),
                None,
                vec![Button::new(
                    uuid!("340917e8-87a9-455c-9645-d08eb99162f1"),
                    "button_0".to_string(),
                    ButtonMode::Hold
                )],
                HashMap::from([(
                    uuid!("340917e8-87a9-455c-9645-d08eb99162f1"),
                    vec![ButtonAction::KeyboardButton(KeyboardButton::Key(Key::A))],
                )])
            ))
        );

        assert_eq!(
            reads_result,
            ClientMessage::Reads(vec![Reads::new(
                Some(mouse::MouseRead::new(100, 100)),
                Some(vec![
                    ButtonAction::MouseButton(MouseButton::Left, MouseButtonState::Up),
                    ButtonAction::KeyboardButton(KeyboardButton::Key(keyboard::Key::A)),
                    ButtonAction::KeyboardButton(KeyboardButton::Sequence(
                        "Hello World!❤️".to_string()
                    )),
                    ButtonAction::KeyboardButton(KeyboardButton::SequenceDsl(
                        "{CTRL+}a{CTRL-}".to_string()
                    ))
                ])
            )])
        );
    }
}
