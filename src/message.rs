use serde::{Deserialize, Serialize};
use std::collections::HashMap;

use crate::device::DeviceId;
use crate::{button, device, gamepad, mouse};

#[derive(Serialize, Deserialize, PartialEq, Debug, Clone)]
#[serde(rename_all = "snake_case")]
pub enum ClientMessage {
    Device(device::Device),
    ButtonActions(Vec<button::ButtonAction>),
    MouseRead(mouse::MouseRead),
    AxisRead(gamepad::AxisRead),
    HatRead(gamepad::HatRead),
}

#[derive(Serialize, Deserialize, PartialEq, Debug, Clone)]
#[serde(rename_all = "snake_case")]
pub enum ServerMessage {
    // TODO: implement a way to config analog inputs
    UpdateDevice(
        DeviceId,
        HashMap<button::ButtonId, Vec<button::ButtonAction>>,
    ),
    RestartDevice(DeviceId),
    ClearCredentials(DeviceId),
}

#[cfg(test)]
mod tests {
    use crate::gamepad::{
        Axis, AxisRead, FourWayHat, GamepadButton, GamepadButtonState, Hat, HatRead, HatValue,
    };
    use crate::{keyboard, keyboard::KeyboardButton};
    use std::collections::HashMap;

    use super::*;
    use crate::button::{Button, ButtonAction, ButtonMode};
    use crate::command::CustomCommand;
    use crate::keyboard::Key;
    use crate::mouse::{MouseButton, MouseButtonState};
    use uuid::uuid;

    #[test]
    fn serialize_client_message() {
        let device_msg = r#"{"device": {"id": "340917e8-87a9-455c-9645-d08eb99162f9","name": "asd","mouse_config": null,"buttons": [{ "id": "340917e8-87a9-455c-9645-d08eb99162f1", "name": "button_0", "mode": "hold" }], "actions_map": { "340917e8-87a9-455c-9645-d08eb99162f1": [{ "keyboard_button": { "key": "A" }}] } }}"#;
        let mouse_msg = r#"{"mouse_read": {"x_read": 100, "y_read": 100} }"#;
        let buttons_msg = r#"{"button_actions": [{"mouse_button": ["left", "up"]}, {"keyboard_button": {"key": "A"}}, {"keyboard_button": {"sequence": "Hello World!❤️"}}, {"keyboard_button": {"sequence_dsl": "{+CTRL}a{-CTRL}"}}, {"gamepad_button": ["button1", "pressed"]}, {"custom_button": {"binary": "C:\\Program Files\\Google\\Chrome\\Application\\chrome.exe"}} ]}"#;
        let axis_msg = r#"{"axis_read": ["axis1", 100] }"#;
        let hat_msg = r#"{"hat_read": ["hat1", {"discrete": "centered"}]}"#;
        let id = uuid!("340917e8-87a9-455c-9645-d08eb99162f9");

        let device_result: ClientMessage = serde_json::from_str(device_msg).unwrap();
        let mouse_result: ClientMessage = serde_json::from_str(mouse_msg).unwrap();
        let button_result: ClientMessage = serde_json::from_str(buttons_msg).unwrap();
        let axis_result: ClientMessage = serde_json::from_str(axis_msg).unwrap();
        let hat_result: ClientMessage = serde_json::from_str(hat_msg).unwrap();

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
            mouse_result,
            ClientMessage::MouseRead(mouse::MouseRead::new(100, 100))
        );

        assert_eq!(
            button_result,
            ClientMessage::ButtonActions(vec![
                ButtonAction::MouseButton(MouseButton::Left, MouseButtonState::Up),
                ButtonAction::KeyboardButton(KeyboardButton::Key(keyboard::Key::A)),
                ButtonAction::KeyboardButton(KeyboardButton::Sequence("Hello World!❤️".to_string())),
                ButtonAction::KeyboardButton(KeyboardButton::SequenceDsl(
                    "{+CTRL}a{-CTRL}".to_string()
                )),
                ButtonAction::GamepadButton(GamepadButton::Button1, GamepadButtonState::Pressed),
                ButtonAction::CustomButton(CustomCommand::Binary(
                    "C:\\Program Files\\Google\\Chrome\\Application\\chrome.exe".to_string()
                ))
            ])
        );

        assert_eq!(
            axis_result,
            ClientMessage::AxisRead(AxisRead::new(Axis::Axis1, 100))
        );

        assert_eq!(
            hat_result,
            ClientMessage::HatRead(HatRead::new(
                Hat::Hat1,
                HatValue::Discrete(FourWayHat::Centered)
            ))
        );
    }
}
