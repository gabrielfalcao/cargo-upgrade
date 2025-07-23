use std::fmt::Display;

use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum Error {
    IOError(String),
    SerializationError(String),
    DeserializationError(String),
    CratesIOError(String),
    CurlError(String),
    TomlError(String),
}
impl Display for Error {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(
            f,
            "{}: {}",
            self.variant(),
            match self {
                Self::IOError(e) => e.to_string(),
                Self::SerializationError(e) => e.to_string(),
                Self::DeserializationError(e) => e.to_string(),
                Self::CratesIOError(e) => e.to_string(),
                Self::CurlError(e) => e.to_string(),
                Self::TomlError(e) => e.to_string(),
            }
        )
    }
}

impl Error {
    pub fn variant(&self) -> String {
        match self {
            Error::IOError(_) => "IOError",
            Error::SerializationError(_) => "SerializationError",
            Error::DeserializationError(_) => "DeserializationError",
            Error::CratesIOError(_) => "CratesIOError",
            Error::CurlError(_) => "CurlError",
            Error::TomlError(_) => "TomlError",
        }
        .to_string()
    }
}

impl std::error::Error for Error {}
impl From<std::io::Error> for Error {
    fn from(e: std::io::Error) -> Self {
        Error::IOError(format!("{}", e))
    }
}
impl From<iocore::Error> for Error {
    fn from(e: iocore::Error) -> Self {
        Error::IOError(format!("{}", e))
    }
}
impl From<toml::de::Error> for Error {
    fn from(e: toml::de::Error) -> Self {
        Error::DeserializationError(format!("{}", e))
    }
}
impl From<toml::ser::Error> for Error {
    fn from(e: toml::ser::Error) -> Self {
        Error::SerializationError(format!("{}", e))
    }
}
impl From<crates_io::Error> for Error {
    fn from(e: crates_io::Error) -> Self {
        Error::CratesIOError(format!("{}", e))
    }
}
impl From<curl::Error> for Error {
    fn from(e: curl::Error) -> Self {
        Error::CurlError(format!("{}", e))
    }
}
impl From<toml_edit::TomlError> for Error {
    fn from(e: toml_edit::TomlError) -> Self {
        Error::TomlError(format!("{}", e))
    }
}
impl From<toml_edit::DatetimeParseError> for Error {
    fn from(e: toml_edit::DatetimeParseError) -> Self {
        Error::TomlError(format!("{}", e))
    }
}
impl From<toml_edit::de::Error> for Error {
    fn from(e: toml_edit::de::Error) -> Self {
        Error::DeserializationError(format!("{}", e))
    }
}
impl From<toml_edit::ser::Error> for Error {
    fn from(e: toml_edit::ser::Error) -> Self {
        Error::SerializationError(format!("{}", e))
    }
}
pub type Result<T> = std::result::Result<T, Error>;
