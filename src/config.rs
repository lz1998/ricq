use std::fmt::Debug;

use serde::{Deserialize, Serialize};

use crate::client::protocol::device::Device;

#[derive(Serialize, Deserialize, Debug)]
pub struct Config {
    pub uin: i64,
    pub password: String,
    pub allow_slice: bool,
    #[serde(skip)]
    pub device: Device,
}

impl Default for Config {
    fn default() -> Self {
        Config {
            uin: 0,
            password: "".to_string(),
            allow_slice: false,
            device: Device::random(),
        }
    }
}

impl Config {
    pub fn load_device_info(&mut self, device: Device) {
        self.device = device;
    }

    pub fn new_with_device_info(device_info: Device) -> Self {
        let mut config = Self::default();
        config.load_device_info(device_info);
        config
    }
}
