use vjoy::VJoy;

use crate::gamepad::{
    Axis, AxisValue, FourWayHat, GamepadButton, GamepadButtonState, Hat, HatValue,
};

impl From<GamepadButtonState> for vjoy::ButtonState {
    fn from(value: GamepadButtonState) -> Self {
        match value {
            GamepadButtonState::Released => vjoy::ButtonState::Released,
            GamepadButtonState::Pressed => vjoy::ButtonState::Pressed,
        }
    }
}

impl From<FourWayHat> for vjoy::FourWayHat {
    fn from(value: FourWayHat) -> Self {
        match value {
            FourWayHat::Centered => vjoy::FourWayHat::Centered,
            FourWayHat::North => vjoy::FourWayHat::North,
            FourWayHat::East => vjoy::FourWayHat::East,
            FourWayHat::South => vjoy::FourWayHat::South,
            FourWayHat::West => vjoy::FourWayHat::West,
        }
    }
}

impl From<HatValue> for vjoy::HatState {
    fn from(value: HatValue) -> Self {
        match value {
            HatValue::Discrete(value) => vjoy::HatState::Discrete(value.into()),
            HatValue::Continuous(value) => vjoy::HatState::Continuous(value),
        }
    }
}

pub type GamePadError = String;

// TODO: implement a basic wrapper for vjoy_driver crate
// TODO: for now we are only implementing one device, we can create up to 16 devices with vjoy
pub trait GamePadAdapter {
    // TODO: think about normalize "adapters", maybe replace set_button with button_up and down like mouse
    fn set_button(
        &mut self,
        button: GamepadButton,
        state: GamepadButtonState,
    ) -> Result<(), GamePadError>;
    fn set_axis(&mut self, axis: Axis, value: AxisValue) -> Result<(), GamePadError>;
    fn set_hat(&mut self, hat: Hat, value: HatValue) -> Result<(), GamePadError>;
}

// TODO: we catch an estrange behavior with Vjoy::from. If we call it, drop it and call it again vjoy in windows throws an error.
// but if we call it two times and save it in different variables we can drop one and call it again and save in a third variable without throw.
// this implies that once you call it, you have to had a copy to be able to drop the variable a be able to call it again later. Something like this:
/*
    // We can do this
    let mut vjoy1 = VJoy::from_default_dll_location().unwrap();
    let mut vjoy2 = VJoy::from_default_dll_location().unwrap();
    drop(vjoy1)
    let mut vjoy3 = VJoy::from_default_dll_location().unwrap();

    // We cant do this
    let mut vjoy1 = VJoy::from_default_dll_location().unwrap();
    let mut vjoy2 = VJoy::from_default_dll_location().unwrap();
    drop(vjoy1)
    drop(vjoy2)
    let mut vjoy3 = VJoy::from_default_dll_location().unwrap();

    // We cant do this
    let mut vjoy1 = VJoy::from_default_dll_location().unwrap();
    drop(vjoy1)
    let mut vjoy2 = VJoy::from_default_dll_location().unwrap();
*/
// One instance of Vjoy must live in order to re instantiate it again
// In order to use this Driver you have to instantiate in a Mutex and drop the MutexGuard
// It can be an Arc<Mutex> or a static Mutex value
pub struct GamepadDriver {
    vjoy: VJoy,
    device: vjoy::Device,
}

impl Default for GamepadDriver {
    fn default() -> Self {
        let vjoy = VJoy::from_default_dll_location().unwrap();
        let device = vjoy.get_device_state(1).unwrap();

        GamepadDriver { vjoy, device }
    }
}

impl GamePadAdapter for GamepadDriver {
    fn set_button(
        &mut self,
        button: GamepadButton,
        state: GamepadButtonState,
    ) -> Result<(), GamePadError> {
        // TODO: manage error
        self.device
            .set_button(button.into(), state.into())
            .expect("[gamepad]: set_button");
        self.vjoy
            .update_device_state(&self.device)
            .expect("[gamepad]: set_button");

        Ok(())
    }

    fn set_axis(&mut self, axis: Axis, value: AxisValue) -> Result<(), GamePadError> {
        self.device
            .set_axis(axis.into(), value)
            .expect("[gamepad]: set_axis");

        self.vjoy
            .update_device_state(&self.device)
            .expect("[gamepad]: set_axis");

        Ok(())
    }

    fn set_hat(&mut self, hat: Hat, value: HatValue) -> Result<(), GamePadError> {
        // TODO: find a way to verify the hat type, to remove responsibility from user
        self.device
            .set_hat(hat.into(), value.into())
            .expect("[gamepad]: set_hat");

        self.vjoy
            .update_device_state(&self.device)
            .expect("[gamepad]: set_hat");

        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use std::time::Duration;

    use super::*;

    const AXIS_MAX: i16 = i16::MAX;
    const AXIS_MID: i16 = i16::MAX / 2;
    const AXIS_MIN: i16 = 0;

    #[test]
    fn vjoy() {
        let mut driver = GamepadDriver::default();

        driver
            .set_button(GamepadButton::Button1, GamepadButtonState::Pressed)
            .unwrap();

        std::thread::sleep(Duration::from_secs(1));

        driver
            .set_button(GamepadButton::Button1, GamepadButtonState::Released)
            .unwrap();

        driver.set_axis(Axis::Axis1, AXIS_MAX.into()).unwrap();

        std::thread::sleep(Duration::from_millis(500));

        driver.set_axis(Axis::Axis1, AXIS_MID.into()).unwrap();

        std::thread::sleep(Duration::from_millis(500));

        driver.set_axis(Axis::Axis1, AXIS_MIN.into()).unwrap();

        driver
            .set_hat(Hat::Hat1, HatValue::Discrete(FourWayHat::North))
            .unwrap();
        std::thread::sleep(Duration::from_millis(500));

        driver
            .set_hat(Hat::Hat1, HatValue::Discrete(FourWayHat::East))
            .unwrap();
        std::thread::sleep(Duration::from_millis(500));

        driver
            .set_hat(Hat::Hat1, HatValue::Discrete(FourWayHat::South))
            .unwrap();
        std::thread::sleep(Duration::from_millis(500));

        driver
            .set_hat(Hat::Hat1, HatValue::Discrete(FourWayHat::West))
            .unwrap();
        std::thread::sleep(Duration::from_millis(500));

        driver
            .set_hat(Hat::Hat1, HatValue::Discrete(FourWayHat::Centered))
            .unwrap();
        std::thread::sleep(Duration::from_millis(500));
    }
}
