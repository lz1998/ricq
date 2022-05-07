use std::io;

use thiserror::Error;

pub type RQResult<T> = Result<T, RQError>;

#[derive(Error, Debug)]
pub enum RQError {
    #[error("other error {0}")]
    Other(String),

    #[error("failed to decode, {0}")]
    Decode(String),

    #[error("From utf-8 error {0}")]
    Utf8(#[from] std::string::FromUtf8Error),

    #[error("command_name mismatch, expected {0} get {1}")]
    CommandNameMismatch(String, String),

    #[error("timeout error")]
    Timeout,

    #[error("network error")]
    Network,

    #[error("jce error, {0}")]
    Jce(#[from] jcers::JceError),
    #[error("io error, {0}")]
    IO(#[from] io::Error),

    #[error("unknown flag {0}")]
    UnknownFlag(u8),

    #[error("unknown encrypt type")]
    UnknownEncryptType,

    #[error("invalid packet type")]
    InvalidPacketType,
    #[error("invalid encrypt type")]
    InvalidEncryptType,
    #[error("packet dropped")]
    PacketDropped,
    #[error("session expired")]
    SessionExpired,
    #[error("session expired, {0}")]
    UnsuccessfulRetCode(i32),

    #[error("Token login failed")]
    TokenLoginFailed,
}
