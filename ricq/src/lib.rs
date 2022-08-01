#![feature(map_first_last)]
#![feature(async_closure)]
#![feature(let_chains)]
#![feature(result_flattening)]

pub use client::handler;
pub use client::Client;
pub use config::Config;
pub use ricq_core::command::wtlogin::{
    LoginDeviceLockLogin, LoginDeviceLocked, LoginNeedCaptcha, LoginResponse, LoginSuccess,
    LoginUnknownStatus, QRCodeConfirmed, QRCodeImageFetch, QRCodeState,
};
pub use ricq_core::error::{RQError, RQResult};
use ricq_core::jce;
pub use ricq_core::msg;
pub use ricq_core::protocol::device;
pub use ricq_core::protocol::version;

pub mod client;
mod config;
pub mod ext;
pub mod structs;
