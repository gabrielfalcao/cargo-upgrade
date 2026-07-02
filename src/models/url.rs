use serde_json::Value;
use std::ops::Deref;

use crate::{Error, Result};
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, PartialOrd, Ord)]
pub struct Url {
    value: String,
}

impl Url {
    pub fn get_validated_url<T: std::string::ToString>(value: T) -> Option<reqwest::Url> {
        let value = value.to_string();
        let validated = reqwest::Url::parse(&value)?;
        Ok(validated)
    }

    pub fn new<T: std::string::ToString>(url: T) -> Result<Url> {
        let value = url.to_string();
        let _ = Url::get_validated_url(url)?;
        Ok(Url { value })
    }
    pub fn as_reqwest(&self) -> reqwest::Url {
        Url::get_validated_url(&self.value)
            .expect("pre-validated reqwest::Url via models::Url::new()")
    }
    pub fn as_url(&self) -> url::Url {
        url::Url::parse(&self.value).expect("pre-validated url::Url via models::Url::new()")
    }
}
impl From<&reqwest::Url> for Url {
    fn from(req: &reqwest::Url) -> Url {
        Url::new(req)
            .expect("should not fail as reqwest::Url is the basis for validating a new crate::Url")
    }
}
impl Into<reqwest::Url> for Url {
    fn into(self) -> reqwest::Url {
        self.as_reqwest()
    }
}
impl From<&url::Url> for Url {
    fn from(req: &url::Url) -> Url {
        Url::new(req).expect("should not fail")
    }
}
impl Into<url::Url> for Url {
    fn into(self) -> url::Url {
        self.as_url()
    }
}
impl Deref for Url {
    type Target = reqwest::Url;

    fn deref(&self) -> &Self::Target {
        &self.as_reqwest()
    }
}
