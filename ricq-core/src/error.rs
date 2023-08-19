use std::io;

use thiserror::Error;

pub type RQResult<T> = Result<T, RQError>;

#[derive(Error, Debug)]
pub enum RQError {
    #[error("other error {0}")]
    Other(String),

    #[error("failed to decode, {0}")]
    Decode(String),

    #[error("failed to decode_prost, {0}")]
    PbDecode(#[from] prost::DecodeError),

    #[error("empty field, {0}")]
    EmptyField(&'static str),

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
    #[error("unsuccessful ret code: {0}")]
    UnsuccessfulRetCode(i32),

    #[error("Token login failed")]
    TokenLoginFailed,
    #[error("failed to get file count")]
    GetFileCountFailed,
    #[error("failed to get file list: {0}")]
    GetFileListFailed(String),
    #[error("crypto invalid length: {0}")]
    CryptoInvalidLength(#[from] crypto_common::InvalidLength),
    #[error("serde_json error: {0}")]
    Serde(#[from] serde_json::Error),
    #[error("block unpad error: {0}")]
    BlockUnpad(#[from] block_padding::UnpadError),
    #[error("spki error: {0}")]
    Spki(#[from] spki::Error),
    #[error("qimei error code: {0}")]
    QimeiError(i64),
    #[error("base64 decode error: {0}")]
    Base64Decode(#[from] base64::DecodeError),
    #[error("rsa error: {0}")]
    RSA(#[from] rsa::Error),
}
