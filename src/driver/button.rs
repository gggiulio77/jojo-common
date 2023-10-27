use crate::keyboard::Key;
use crate::{driver, mouse};
use enigo::Enigo;

use super::mouse::EnigoMouseButton;

pub struct ButtonDriver {
    mouse_driver: Box<dyn driver::mouse::MouseAdapter + Send + Sync>,
    keyboard_driver: Box<dyn driver::keyboard::KeyboardAdapter + Send + Sync>,
}

impl Default for ButtonDriver {
    fn default() -> Self {
        ButtonDriver {
            mouse_driver: Box::new(Enigo::new()),
            keyboard_driver: Box::new(Enigo::new()),
        }
    }
}

impl<'a> ButtonDriver {
    pub fn new(
        mouse_driver: Box<dyn driver::mouse::MouseAdapter + Send + Sync>,
        keyboard_driver: Box<dyn driver::keyboard::KeyboardAdapter + Send + Sync>,
    ) -> ButtonDriver {
        ButtonDriver {
            mouse_driver,
            keyboard_driver,
        }
    }

    fn decode_mouse_button(
        &self,
        mouse_button: &'a mouse::MouseButton,
    ) -> (EnigoMouseButton, &'a mouse::MouseButtonState) {
        match mouse_button {
            mouse::MouseButton::Left(state) => (EnigoMouseButton::Left, state),
            mouse::MouseButton::Right(state) => (EnigoMouseButton::Right, state),
            mouse::MouseButton::Back(state) => (EnigoMouseButton::Back, state),
            mouse::MouseButton::Forward(state) => (EnigoMouseButton::Forward, state),
            mouse::MouseButton::Middle(state) => (EnigoMouseButton::Middle, state),
            mouse::MouseButton::ScrollDown(state) => (EnigoMouseButton::ScrollDown, state),
            mouse::MouseButton::ScrollLeft(state) => (EnigoMouseButton::ScrollLeft, state),
            mouse::MouseButton::ScrollRight(state) => (EnigoMouseButton::ScrollRight, state),
            mouse::MouseButton::ScrollUp(state) => (EnigoMouseButton::ScrollUp, state),
        }
    }

    // TODO: think a better way to do this, maybe a impl into o something similar
    pub fn mouse_button_to_state(&mut self, mouse_button: &mouse::MouseButton) {
        let (button, state) = self.decode_mouse_button(mouse_button);
        match state {
            mouse::MouseButtonState::Up => self.mouse_driver.mouse_move_up(button),
            mouse::MouseButtonState::Down => self.mouse_driver.mouse_move_down(button),
        }
    }

    pub fn key_sequence(&mut self, sequence: &str) {
        self.keyboard_driver.key_sequence(sequence)
    }

    pub fn key_sequence_dsl(&mut self, sequence: &str) {
        self.keyboard_driver.key_sequence_parse(sequence)
    }
    pub fn key_click(&mut self, key: Key) {
        self.keyboard_driver.key_click(key)
    }
}
