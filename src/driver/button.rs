use crate::keyboard::Key;
use crate::{gamepad, mouse};

use super::gamepad::{GamePadAdapter, GamepadDriver};
use super::keyboard::KeyboardDriver;
use super::mouse::{MouseAdapter, MouseDriver};

#[derive(Default)]
pub struct ButtonDriver {
    mouse_driver: MouseDriver,
    keyboard_driver: KeyboardDriver,
    gamepad_driver: GamepadDriver,
}

impl ButtonDriver {
    pub fn new(
        mouse_driver: MouseDriver,
        keyboard_driver: KeyboardDriver,
        gamepad_driver: GamepadDriver,
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
