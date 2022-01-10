use thiserror::Error;

pub type RQResult<T> = Result<T, RQError>;

#[derive(Error, Debug)]
pub enum RQError {
    #[error("other error {0}")]
    Other(String),

    #[error("failed to decode, {0}")]
    Decode(String),

    #[error("command_name is error, {0}")]
    CommandName(String),

    #[error("timeout error")]
    Timeout,

    #[error("network error")]
    Network,

    #[error("jce error, {0}")]
    Jce(#[from] jcers::JceError),
}
