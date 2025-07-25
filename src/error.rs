use std::{error, fmt::Display};

use serde::{de, ser};

pub type Result<T, E = Error> = std::result::Result<T, E>;

#[derive(Debug)]
pub enum Error {
    /// Custom error created by serde
    Custom(String),
    Io(std::io::Error),
    /// Produced when an unsupported function is called
    Unsupported {
        name: &'static str,
        reason: &'static str,
    },
    /// Produced when an unexpected EOF is found
    EOF,
    /// Produced when an invalid value is found
    InvalidValue {
        value: u32,
        reason: &'static str,
    },
    /// Produced when an unsupported version of SBOF is attempted to be deserialized
    UnsupportedVersion,
    /// Produced when an invalid UTF-8 string is found
    InvalidUTF8,
}

impl From<std::io::Error> for Error {
    fn from(value: std::io::Error) -> Self {
        Self::Io(value)
    }
}

impl ser::Error for Error {
    fn custom<T>(msg: T) -> Self
    where
        T: std::fmt::Display,
    {
        Self::Custom(msg.to_string())
    }
}

impl de::Error for Error {
    fn custom<T>(msg: T) -> Self
    where
        T: std::fmt::Display,
    {
        Self::Custom(msg.to_string())
    }
}

impl Display for Error {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        use Error::*;

        match self {
            Custom(str) => write!(f, "{str}"),
            Unsupported { name, reason } => {
                write!(f, "unsupported function {name} called. {reason}")
            }
            Io(e) => write!(f, "{e}"),
            EOF => write!(f, "unexpected eof"),
            InvalidValue { value, reason } => write!(f, "invalid value {value}, {reason}"),
            UnsupportedVersion => write!(f, "tried to deserialize unsupported version of SBOF"),
            InvalidUTF8 => write!(f, "tried to parse invalid UTF-8"),
        }
    }
}

impl error::Error for Error {}
