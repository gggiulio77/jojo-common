use serde::{Deserialize, Serialize};

use crate::driver::keyboard::EnigoKey;

// TODO: this brings EnigoKey, is not good but the Key enum es HUGE, maybe we can made it generic or some other shit
// TODO: or maybe we can move all enigo shit related to /driver and use a feature flag
#[derive(Debug, Clone, Deserialize, Serialize, Eq, PartialEq)]
#[serde(rename_all = "snake_case")]
pub enum KeyboardButton {
    Sequence(String),
    SequenceDsl(String),
    Key(EnigoKey),
}
