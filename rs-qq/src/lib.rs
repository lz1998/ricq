#![feature(map_first_last)]
#![feature(async_closure)]
#![feature(let_chains)]

pub use client::handler;
pub use client::Client;
pub use config::Config;
pub use engine::command::wtlogin::{
    LoginDeviceLockLogin, LoginDeviceLocked, LoginNeedCaptcha, LoginResponse, LoginSuccess,
    LoginUnknownStatus, QRCodeConfirmed, QRCodeImageFetch, QRCodeState,
};
pub use engine::error::{RQError, RQResult};
use engine::jce;
pub use engine::msg;
pub use engine::protocol::device;
pub use engine::protocol::version;
use rq_engine as engine;

pub mod client;
mod config;
pub mod ext;
pub mod structs;
