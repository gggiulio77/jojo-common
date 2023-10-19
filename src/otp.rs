use crate::network::Ssid;
use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize, Serialize, Clone, Default)]
pub struct ScanResponse {
    found_ssid: Vec<Ssid>,
}

impl ScanResponse {
    pub fn new(found_ssid: Vec<Ssid>) -> Self {
        ScanResponse { found_ssid }
    }
    pub fn found_ssid(&self) -> &Vec<Ssid> {
        &self.found_ssid
    }
}
