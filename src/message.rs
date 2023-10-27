use serde::{Deserialize, Serialize};

use crate::{button, device, mouse};

#[derive(Serialize, Deserialize, PartialEq, Debug)]
#[serde(rename_all = "snake_case")]
pub struct Reads {
    mouse_read: Option<mouse::MouseRead>,
    button_reads: Option<Vec<button::ButtonRead>>,
}

impl Reads {
    pub fn new(
        mouse_read: Option<mouse::MouseRead>,
        button_reads: Option<Vec<button::ButtonRead>>,
    ) -> Self {
        Reads {
            mouse_read,
            button_reads,
        }
    }

    pub fn mouse_read(&self) -> &Option<mouse::MouseRead> {
        &self.mouse_read
    }

    pub fn button_reads(&self) -> &Option<Vec<button::ButtonRead>> {
        &self.button_reads
    }
}

#[derive(Serialize, Deserialize, PartialEq, Debug)]
#[serde(rename_all = "snake_case")]
pub enum ClientMessage {
    Reads(Vec<Reads>),
    Device(device::Device),
}

#[cfg(feature = "driver")]
#[cfg(test)]
mod tests {
    use crate::{
        button::ButtonRead,
        driver::{self},
        keyboard,
        keyboard::KeyboardButton,
    };

    use super::*;
    use uuid::uuid;

    #[test]
    fn test_serialize_message() {
        let device_msg = r#"{"device": {"id": "340917e8-87a9-455c-9645-d08eb99162f9","name": "asd","mouse_config": null,"buttons": []}}"#;
        let reads_msg = r#"{"reads": [{"mouse_read": {"x_read": 100, "y_read": 100}, "button_reads": [{"keyboard_button": {"key": "A"}}, {"keyboard_button": {"sequence": "Hello World!❤️"}}, {"keyboard_button": {"sequence_dsl": "{CTRL+}a{CTRL-}"}}]}]}"#;
        let id = uuid!("340917e8-87a9-455c-9645-d08eb99162f9");

        let device_result: ClientMessage = serde_json::from_str(device_msg).unwrap();
        let reads_result: ClientMessage = serde_json::from_str(reads_msg).unwrap();

        assert_eq!(
            device_result,
            ClientMessage::Device(device::Device::new(id, format!("asd"), None, Vec::new()))
        );

        assert_eq!(
            reads_result,
            ClientMessage::Reads(vec![Reads::new(
                Some(mouse::MouseRead::new(100, 100)),
                Some(vec![
                    ButtonRead::KeyboardButton(KeyboardButton::Key(keyboard::Key::A)),
                    ButtonRead::KeyboardButton(KeyboardButton::Sequence(
                        "Hello World!❤️".to_string()
                    )),
                    ButtonRead::KeyboardButton(KeyboardButton::SequenceDsl(
                        "{CTRL+}a{CTRL-}".to_string()
                    ))
                ])
            )])
        );
    }
}
