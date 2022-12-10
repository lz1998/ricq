#![feature(async_closure)]
#![feature(let_chains)]
#![feature(result_flattening)]

pub mod client;
mod config;
pub mod ext;
pub mod structs;

pub use client::handler;
pub use client::Client;
pub use config::Config;
pub use device::Device;
pub use version::Protocol;

pub use ricq_core::command::wtlogin::{
    LoginDeviceLockLogin, LoginDeviceLocked, LoginNeedCaptcha, LoginResponse, LoginSuccess,
    LoginUnknownStatus, QRCodeConfirmed, QRCodeImageFetch, QRCodeState,
};
pub use ricq_core::error::{RQError, RQResult};
use ricq_core::jce;
pub use ricq_core::msg;
pub use ricq_core::protocol::device;
pub use ricq_core::protocol::version;
