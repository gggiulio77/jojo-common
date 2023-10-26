use enigo::{Enigo, MouseControllable};
pub type EnigoMouseButton = enigo::MouseButton;

pub trait MouseAdapter {
    fn mouse_move_relative(&mut self, x: i32, y: i32);
    fn mouse_move_down(&mut self, button: EnigoMouseButton);
    fn mouse_move_up(&mut self, button: EnigoMouseButton);
}

impl MouseAdapter for Enigo {
    fn mouse_move_relative(&mut self, x: i32, y: i32) {
        // println!("moving mouse {x} horizontal and {y} vertical");
        MouseControllable::mouse_move_relative(self, x, y)
    }
    fn mouse_move_down(&mut self, button: EnigoMouseButton) {
        MouseControllable::mouse_down(self, button)
    }
    fn mouse_move_up(&mut self, button: EnigoMouseButton) {
        MouseControllable::mouse_up(self, button)
    }
}

// TODO: find a way to derive Clone, Enigo does not impl CLONE (mayor fuck)
pub struct MouseDriver(Box<dyn MouseAdapter + Send + Sync>);

impl Default for MouseDriver {
    fn default() -> MouseDriver {
        MouseDriver(Box::new(Enigo::new()))
    }
}

impl MouseDriver {
    pub fn new(driver: Box<dyn MouseAdapter + Send + Sync>) -> MouseDriver {
        MouseDriver(driver)
    }

    pub fn mouse_move_relative(&mut self, x: i32, y: i32) {
        self.0.mouse_move_relative(x, y)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn move_pointer() {
        let mut mouse_driver = MouseDriver::default();

        mouse_driver.mouse_move_relative(50, 50);
    }
}
