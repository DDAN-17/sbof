use std::{error, fmt::Display};

use serde::{ser, de};

pub type Result<T> = std::result::Result<T, Error>;

#[derive(Debug)]
pub enum Error {
    Custom(String),
    Io(std::io::Error),
    Unsupported,
}

impl From<std::io::Error> for Error {
    fn from(value: std::io::Error) -> Self {
        Self::Io(value)
    }
}

impl ser::Error for Error {
    fn custom<T>(msg: T) -> Self
    where T:std::fmt::Display {
        Self::Custom(msg.to_string())
    }
}

impl de::Error for Error {
    fn custom<T>(msg: T) -> Self
    where T:std::fmt::Display {
        Self::Custom(msg.to_string())
    }
}

impl Display for Error {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        use Error::*;

        match self {
            Custom(str) => write!(f, "{str}"),
            Unsupported => write!(f, "unsupported function called"),
            Io(e) => write!(f, "{e}"),
        }
    }
}

impl error::Error for Error {}