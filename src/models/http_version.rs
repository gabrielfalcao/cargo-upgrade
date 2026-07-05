use crate::{Error, Result};
use http::Version;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct HttpVersion {
    version: String,
}

impl HttpVersion {
    pub fn new(version: String) -> HttpVersion {
        HttpVersion { version }
    }
    pub fn from_reqwest(version: reqwest::Version) -> Result<HttpVersion> {
        use reqwest::Version;
        let version = if version == Version::HTTP_09 {
            "0.9".to_string()
        } else if version == Version::HTTP_10 {
            "1.0".to_string()
        } else if version == Version::HTTP_11 {
            "1.1".to_string()
        } else if version == Version::HTTP_2 {
            "2.0".to_string()
        } else if version == Version::HTTP_3 {
            "3.0".to_string()
        } else {
            return Err(Error::HttpError(format!(
                "unknown http version: {version:#?}"
            )));
        };
        Ok(HttpVersion { version })
    }
    pub fn to_reqwest(&self) -> Option<reqwest::Version> {
        let version = match self.version.as_str() {
            "0.9" => Version::HTTP_09,
            "1.0" => Version::HTTP_10,
            "1.1" => Version::HTTP_10,
            "2.0" => Version::HTTP_10,
            "3.0" => Version::HTTP_10,
            _unknown => return None,
        };
        Some(version)
    }
}
impl std::fmt::Display for HttpVersion {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{}", &self.version)
    }
}

impl From<reqwest::Version> for HttpVersion {
    fn from(v: reqwest::Version) -> HttpVersion {
        HttpVersion::from_reqwest(v).expect("valid http version")
    }
}
