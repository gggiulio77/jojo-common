use crate::keyboard::Key;
use crate::{driver, gamepad, mouse};
use enigo::Enigo;

use super::gamepad::GamepadDriver;

pub struct ButtonDriver {
    mouse_driver: Box<dyn driver::mouse::MouseAdapter + Send + Sync>,
    keyboard_driver: Box<dyn driver::keyboard::KeyboardAdapter + Send + Sync>,
    gamepad_driver: Box<dyn driver::gamepad::GamePadAdapter + Send + Sync>,
}

impl Default for ButtonDriver {
    fn default() -> Self {
        ButtonDriver {
            mouse_driver: Box::new(Enigo::new()),
            keyboard_driver: Box::new(Enigo::new()),
            gamepad_driver: Box::new(GamepadDriver::default()),
        }
    }
}

impl ButtonDriver {
    pub fn new(
        mouse_driver: Box<dyn driver::mouse::MouseAdapter + Send + Sync>,
        keyboard_driver: Box<dyn driver::keyboard::KeyboardAdapter + Send + Sync>,
        gamepad_driver: Box<dyn driver::gamepad::GamePadAdapter + Send + Sync>,
    ) -> ButtonDriver {
        ButtonDriver {
            mouse_driver,
            keyboard_driver,
            gamepad_driver,
        }
    }

    pub fn mouse_button_to_state(
        &mut self,
        mouse_button: mouse::MouseButton,
        state: mouse::MouseButtonState,
    ) {
        match state {
            mouse::MouseButtonState::Up => self.mouse_driver.mouse_move_up(mouse_button),
            mouse::MouseButtonState::Down => self.mouse_driver.mouse_move_down(mouse_button),
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

    pub fn gamepad_button_to_state(
        &mut self,
        gamepad_button: gamepad::GamepadButton,
        state: gamepad::GamepadButtonState,
    ) {
        self.gamepad_driver
            .set_button(gamepad_button, state)
            .expect("[gamepad_button_to_state]: error")
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::keyboard;

    #[test]
    fn mouse_test() {
        let mut driver = ButtonDriver::default();

        driver.mouse_button_to_state(mouse::MouseButton::Left, mouse::MouseButtonState::Down);
        driver.mouse_button_to_state(mouse::MouseButton::Left, mouse::MouseButtonState::Up);
    }

    #[test]
    fn keyboard_test() {
        let mut driver = ButtonDriver::default();

        driver.key_click(keyboard::Key::A);
    }
}
