use std::fmt::Debug;

use crate::engine::protocol::{
    device::Device,
    version::Version,
    version::{get_version, Protocol},
};

#[derive(Debug)]
pub struct Config {
    pub device: Device,
    pub version: &'static Version,
}

impl Default for Config {
    fn default() -> Self {
        Self {
            device: Device::random(),
            version: get_version(Protocol::IPad),
        }
    }
}

impl Config {
    pub fn new(device: Device, version: &'static Version) -> Self {
        Self { device, version }
    }
}
