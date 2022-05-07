#![feature(map_first_last)]
#![feature(async_closure)]
#![feature(let_chains)]

pub use client::handler;
pub use client::Client;
pub use config::Config;
pub use oicq_core::command::wtlogin::{
    LoginDeviceLockLogin, LoginDeviceLocked, LoginNeedCaptcha, LoginResponse, LoginSuccess,
    LoginUnknownStatus, QRCodeConfirmed, QRCodeImageFetch, QRCodeState,
};
pub use oicq_core::error::{RQError, RQResult};
use oicq_core::jce;
pub use oicq_core::msg;
pub use oicq_core::protocol::device;
pub use oicq_core::protocol::version;

pub mod client;
mod config;
pub mod ext;
pub mod structs;
