use std::fmt::{self};

use std::time::SystemTimeError;
use tokio::io::Error as TokioError;

#[derive(Debug)]
pub enum Error {
    SystemTime(SystemTimeError),
    Tokio(TokioError),
    Custom(String),
}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Error::Tokio(err) => write!(f, "Tokio error: {}", err),
            Error::SystemTime(err) => write!(f, "SystemTimeError: {}", err),
            Error::Custom(err) => write!(f, "{}", err),
        }
    }
}

impl From<SystemTimeError> for Error {
    fn from(err: SystemTimeError) -> Self {
        Self::SystemTime(err)
    }
}

impl From<TokioError> for Error {
    fn from(err: TokioError) -> Self {
        Self::Tokio(err)
    }
}
