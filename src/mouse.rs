use serde::{Deserialize, Serialize};

#[derive(Clone, Copy, Debug, Deserialize, Serialize, PartialEq)]
pub struct MouseRead {
    x_read: i32,
    y_read: i32,
}

impl MouseRead {
    pub fn new(x_read: i32, y_read: i32) -> Self {
        MouseRead { x_read, y_read }
    }
    pub fn x_read(&self) -> i32 {
        self.x_read
    }
    pub fn y_read(&self) -> i32 {
        self.y_read
    }
}

#[derive(Debug, Clone, Deserialize, Serialize, Eq, PartialEq)]
pub struct MouseConfig {
    x_sen: i8,
    y_sen: i8,
}

impl Default for MouseConfig {
    fn default() -> Self {
        MouseConfig {
            x_sen: 1,
            y_sen: -1,
        }
    }
}

impl MouseConfig {
    fn new(x_sen: i8, y_sen: i8) -> Self {
        MouseConfig { x_sen, y_sen }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
}
