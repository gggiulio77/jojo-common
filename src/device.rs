use crate::{button, mouse};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

pub type DeviceId = uuid::Uuid;

#[derive(Debug, Clone, Default, Deserialize, Serialize, Eq, PartialEq)]
#[serde(rename_all = "snake_case")]
pub struct Device {
    id: DeviceId,
    name: String,
    mouse_config: Option<mouse::MouseConfig>,
    // TODO: maybe use a hash or static array
    buttons: Vec<button::Button>,
    actions_map: HashMap<button::ButtonId, button::ButtonAction>,
}

impl Device {
    pub fn new(
        id: DeviceId,
        name: String,
        mouse_config: Option<mouse::MouseConfig>,
        buttons: Vec<button::Button>,
        actions_map: HashMap<button::ButtonId, button::ButtonAction>,
    ) -> Self {
        Device {
            id,
            name,
            mouse_config,
            buttons,
            actions_map,
        }
    }

    pub fn id(&self) -> DeviceId {
        self.id
    }

    pub fn name(&self) -> &str {
        &self.name
    }

    pub fn mouse_config(&self) -> &Option<mouse::MouseConfig> {
        &self.mouse_config
    }

    pub fn buttons(&self) -> &Vec<button::Button> {
        &self.buttons
    }

    pub fn actions_map(&self) -> &HashMap<button::ButtonId, button::ButtonAction> {
        &self.actions_map
    }

    pub fn set_name(&mut self, name: String) {
        self.name = name;
    }

    pub fn set_mouse_config(&mut self, mouse_config: Option<mouse::MouseConfig>) {
        self.mouse_config = mouse_config;
    }

    pub fn set_actions_map(
        &mut self,
        actions_map: HashMap<button::ButtonId, button::ButtonAction>,
    ) {
        self.actions_map = actions_map;
    }
}
