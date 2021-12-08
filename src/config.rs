use std::fmt::Debug;

use serde::{Deserialize, Serialize};

use crate::client::device::DeviceInfo;

#[derive(Serialize, Deserialize, Debug)]
pub struct Config {
    pub uin: i64,
    pub password: String,
    pub allow_slice: bool,
    #[serde(skip)]
    pub device_info: DeviceInfo,
}

impl Default for Config {
    fn default() -> Self {
        Config {
            uin: 0,
            password: "".to_string(),
            allow_slice: false,
            device_info: DeviceInfo::default(),
        }
    }
}

impl Config {
    pub fn load_device_info(&mut self, device_info: DeviceInfo) {
        self.device_info = device_info;
    }

    pub fn new_with_device_info(device_info: DeviceInfo) -> Self {
        let mut config = Self::default();
        config.load_device_info(device_info);
        config
    }
}
