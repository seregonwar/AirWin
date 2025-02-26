use std::fmt;
use thiserror::Error;

#[derive(Error, Debug)]
pub enum AirWinError {
    #[error("Network error: {0}")]
    NetworkError(String),

    #[error("Device discovery error: {0}")]
    DiscoveryError(String),

    #[error("Protocol error: {0}")]
    ProtocolError(String),

    #[error("Connection timeout")]
    ConnectionTimeout,

    #[error("Invalid network interface: {0}")]
    InvalidInterface(String),

    #[error(transparent)]
    Other(#[from] anyhow::Error),
}

impl AirWinError {
    pub fn is_temporary(&self) -> bool {
        matches!(self, 
            AirWinError::ConnectionTimeout |
            AirWinError::NetworkError(_)
        )
    }

    pub fn should_retry(&self) -> bool {
        self.is_temporary()
    }
}

pub type AirWinResult<T> = Result<T, AirWinError>;
