use serde::{Deserialize, Serialize};

// TODO: for now we are using a default configuration, we need to investigate a way to configure vjoy from the backend
// Default vjoy configuration
// Buttons: 8 -> ids [1..8]
// Axes: 8 -> names: X, Y, Z, Rx, Ry, Rz, Slider, Dial/Slider2 -> ids [1..8]
// POVs: 1, Discrete

pub type ButtonId = u8;
pub type AxisId = u32;
pub type HatID = u8;
pub type AxisValue = i32;

#[derive(Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Default, Deserialize, Serialize)]
#[serde(rename_all = "snake_case")]
pub enum GamepadButtonState {
    #[default]
    Released = 0,
    Pressed,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Deserialize, Serialize)]
#[serde(rename_all = "snake_case")]
pub enum GamepadButton {
    Button1 = 1,
    Button2,
    Button3,
    Button4,
    Button5,
    Button6,
    Button7,
    Button8,
}

impl From<GamepadButton> for ButtonId {
    fn from(value: GamepadButton) -> Self {
        match value {
            GamepadButton::Button1 => 1,
            GamepadButton::Button2 => 2,
            GamepadButton::Button3 => 3,
            GamepadButton::Button4 => 4,
            GamepadButton::Button5 => 5,
            GamepadButton::Button6 => 6,
            GamepadButton::Button7 => 7,
            GamepadButton::Button8 => 8,
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Deserialize, Serialize)]
#[serde(rename_all = "snake_case")]
pub enum Axis {
    Axis1 = 1,
    Axis2,
    Axis3,
    Axis4,
    Axis5,
    Axis6,
    Axis7,
    Axis8,
}

impl From<Axis> for AxisId {
    fn from(value: Axis) -> Self {
        match value {
            Axis::Axis1 => 1,
            Axis::Axis2 => 2,
            Axis::Axis3 => 3,
            Axis::Axis4 => 4,
            Axis::Axis5 => 5,
            Axis::Axis6 => 6,
            Axis::Axis7 => 7,
            Axis::Axis8 => 8,
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Deserialize, Serialize)]
#[serde(rename_all = "snake_case")]
pub enum Hat {
    Hat1,
}

impl From<Hat> for HatID {
    fn from(value: Hat) -> Self {
        match value {
            Hat::Hat1 => 1,
        }
    }
}

#[derive(
    Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Default, Serialize, Deserialize,
)]
#[serde(rename_all = "snake_case")]
pub enum FourWayHat {
    #[default]
    Centered = -1,
    North = 0,
    East = 1,
    South = 2,
    West = 3,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Deserialize, Serialize)]
#[serde(rename_all = "snake_case")]
pub enum HatValue {
    Discrete(FourWayHat),
    Continuous(u32),
}

#[derive(Clone, Copy, Debug, Deserialize, Serialize, PartialEq)]
#[serde(rename_all = "snake_case")]
pub struct AxisRead(pub Axis, pub AxisValue);

impl AxisRead {
    pub fn new(axis: Axis, value: AxisValue) -> Self {
        Self(axis, value)
    }
}

#[derive(Clone, Copy, Debug, Deserialize, Serialize, PartialEq)]
#[serde(rename_all = "snake_case")]
pub struct HatRead(pub Hat, pub HatValue);

impl HatRead {
    pub fn new(hat: Hat, value: HatValue) -> Self {
        Self(hat, value)
    }
}
