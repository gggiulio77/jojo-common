use serde::{Deserialize, Serialize};

use crate::{button, device, mouse};

#[derive(Serialize, Deserialize, PartialEq, Debug)]
pub struct Reads {
    mouse_read: Option<mouse::MouseRead>,
    button_reads: Option<Vec<button::ButtonRead>>,
}

impl Reads {
    pub fn new(
        mouse_read: Option<mouse::MouseRead>,
        button_reads: Option<Vec<button::ButtonRead>>,
    ) -> Self {
        Reads {
            mouse_read,
            button_reads,
        }
    }
}

#[derive(Serialize, Deserialize, PartialEq, Debug)]
pub enum ClientMessage {
    Reads(Vec<Reads>),
    Device(device::Device),
}
