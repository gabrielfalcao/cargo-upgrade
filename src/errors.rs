use std::fmt::Display;

use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum Error {
    IOError(String),
    RuntimeError(String),
    SerializationError(String),
    DeserializationError(String),
    CratesIOError(String),
    HttpError(String),
    TomlError(String),
    ParseError(String),
}
impl Display for Error {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(
            f,
            "{}: {}",
            self.variant(),
            match self {
                Self::IOError(e) => e.to_string(),
                Self::RuntimeError(e) => e.to_string(),
                Self::SerializationError(e) => e.to_string(),
                Self::DeserializationError(e) => e.to_string(),
                Self::CratesIOError(e) => e.to_string(),
                Self::HttpError(e) => e.to_string(),
                Self::TomlError(e) => e.to_string(),
                Self::ParseError(e) => e.to_string(),
            }
        )
    }
}

impl Error {
    pub fn variant(&self) -> String {
        match self {
            Error::IOError(_) => "IOError",
            Error::RuntimeError(_) => "RuntimeError",
            Error::SerializationError(_) => "SerializationError",
            Error::DeserializationError(_) => "DeserializationError",
            Error::CratesIOError(_) => "CratesIOError",
            Error::HttpError(_) => "HttpError",
            Error::TomlError(_) => "TomlError",
            Error::ParseError(_) => "ParseError",
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
impl <T: std::fmt::Display>From<crates_io::Error<T>> for Error {
    fn from(e: crates_io::Error<T>) -> Self {
        Error::CratesIOError(format!("{}", e))
    }
}
impl From<curl::Error> for Error {
    fn from(e: curl::Error) -> Self {
        Error::HttpError(format!("{}", e))
    }
}
impl From<reqwest::Error> for Error {
    fn from(e: reqwest::Error) -> Self {
        Error::HttpError(format!("{}", e))
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
impl From<std::num::ParseIntError> for Error {
    fn from(e: std::num::ParseIntError) -> Self {
        Error::ParseError(format!("{}", e))
    }
}
impl From<std::string::FromUtf8Error> for Error {
    fn from(e: std::string::FromUtf8Error) -> Self {
        Error::ParseError(format!("{}", e))
    }
}

impl From<url::ParseError> for Error {
    fn from(e: url::ParseError) -> Self {
        Error::ParseError(format!("{}", e))
    }
}
impl From<serde_json::Error> for Error {
    fn from(e: serde_json::Error) -> Self {
        Error::ParseError(format!("{}", e))
    }
}
impl From<slugify_filenames::Error> for Error {
    fn from(e: slugify_filenames::Error) -> Self {
        Error::ParseError(format!("{}", e))
    }
}
impl From<sanitation::Error<'_>> for Error {
    fn from(e: sanitation::Error<'_>) -> Self {
        Error::ParseError(format!("{}", e))
    }
}
impl From<color_eyre::Report> for Error {
    fn from(e: color_eyre::Report) -> Self {
        log::error!("{e}");
        Error::RuntimeError(format!("{}", e))
    }
}
pub type Result<T> = std::result::Result<T, Error>;
