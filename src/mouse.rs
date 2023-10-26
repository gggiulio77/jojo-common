use serde::{Deserialize, Serialize};

#[derive(Clone, Copy, Debug, Deserialize, Serialize, PartialEq)]
#[serde(rename_all = "snake_case")]
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
#[serde(rename_all = "snake_case")]
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

    pub fn x_sen(&self) -> i8 {
        self.x_sen
    }
    pub fn y_sen(&self) -> i8 {
        self.y_sen
    }
}

#[derive(Debug, Clone, Copy, Deserialize, Serialize, Eq, PartialEq)]
#[serde(rename_all = "snake_case")]
pub enum MouseButtonState {
    Up,
    Down,
}

#[derive(Debug, Clone, Deserialize, Serialize, Eq, PartialEq)]
#[serde(rename_all = "snake_case")]
pub enum MouseButton {
    Left(MouseButtonState),
    Middle(MouseButtonState),
    Right(MouseButtonState),
    Back(MouseButtonState),
    Forward(MouseButtonState),

    ScrollUp(MouseButtonState),
    ScrollDown(MouseButtonState),
    ScrollLeft(MouseButtonState),
    ScrollRight(MouseButtonState),
}
