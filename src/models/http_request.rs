use std::collections::BTreeMap;

use crate::{HttpVersion, ObjectInfo, Result};
use iocore::Path;
use reqwest::blocking::Request;
use sanitation::SString;
use serde::{Deserialize, Serialize};
use serde_json::Value;
use slugify_filenames::slugify_string;
use std::{
    fmt::{Debug, Display},
    time::Duration,
};
use url::Url;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct HttpRequest {
    method: String,
    url: String,
    body: Vec<u8>,
    headers: BTreeMap<String, String>,
    timeout: Duration,
    version: String,
}

impl HttpRequest {
    pub fn info(&self) -> ObjectInfo<HttpRequest> {
        ObjectInfo {
            value: self.clone(),
            // versions: Value::Array(vec![Value::String(self.version.clone())]),
        }
    }
    pub fn url(&self) -> String {
        self.url.clone()
    }

    pub fn headers(&self) -> BTreeMap<String, String> {
        self.headers.clone()
    }

    pub fn get_header(&self, key: impl Into<String>) -> Option<String> {
        self.headers.get(&key.into()).cloned()
    }

    pub fn content_type(&self) -> Option<String> {
        self.headers.get("content-type").cloned()
    }

    pub fn text(&self) -> Result<String> {
        Ok(self.sanitized_string()?.safe()?)
    }

    pub fn sanitized_string(&self) -> Result<SString> {
        let sstring = SString::new(&self.body);
        sstring.safe()?;
        Ok(sstring)
    }

    pub fn bytes(&self) -> Result<Vec<u8>> {
        Ok(self.text()?.as_bytes().to_vec())
    }
}

impl From<&Request> for HttpRequest {
    fn from(req: &Request) -> HttpRequest {
        let method = req.method().as_str().to_string();
        let url = req.url().to_string();
        let body = req
            .body()
            .map(|body| {
                body.as_bytes()
                    .map(|bytes| bytes.to_vec())
                    .unwrap_or_default()
            })
            .unwrap_or_default();
        let mut headers = BTreeMap::<String, String>::new();
        for (name, value) in req.headers().into_iter() {
            headers.insert(
                name.to_string(),
                value.to_str().map(|v| v.to_string()).unwrap_or_default(),
            );
        }
        let timeout = req.timeout().map(|t| t.clone()).unwrap_or_default();
        let http_version = HttpVersion::from(req.version());
        let version = http_version.to_string();
        HttpRequest {
            method,
            url,
            body,
            headers,
            timeout,
            version,
        }
    }
}
impl Display for HttpRequest {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        let json = serde_json::to_string_pretty(&self).unwrap();
        write!(f, "{json}")
    }
}
