use std::collections::BTreeMap;

use crate::{CookieJar, Result};
use http::HeaderMap;
use iocore::Path;
use reqwest::blocking::Response;
use sanitation::SString;
use serde::{Deserialize, Serialize};
use serde_json::Value;
use slugify_filenames::slugify_string;
use url::Url;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct HttpResponse {
    cookies: CookieJar,
    error: Option<String>,
    body: Vec<u8>,
    url: Url,
    status_code: u16,
    headers: BTreeMap<String, String>,
}

impl HttpResponse {
    pub fn status(&self) -> u16 {
        self.status_code.clone()
    }

    pub fn url(&self) -> Url {
        self.url.clone()
    }

    pub fn url_path(&self) -> Url {
        self.origin().join(self.url().path()).unwrap()
    }

    pub fn origin(&self) -> Url {
        Url::parse(&self.url().origin().ascii_serialization()).unwrap()
    }

    pub fn slug(&self) -> String {
        self.url_path_slug().expect("no errors")
    }
    pub fn url_path_slug(&self) -> Result<String> {
        Ok(slugify_string(self.url_path(), true)?)
    }

    pub fn path(&self) -> Path {
        Path::from(self.slug())
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

    pub fn cookies(&self) -> CookieJar {
        self.cookies.clone()
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

    pub fn error(&self) -> Option<String> {
        self.error.clone()
    }
}

impl From<Response> for HttpResponse {
    fn from(p: Response) -> HttpResponse {
        let mut cookies = CookieJar::new();
        let mut headers = BTreeMap::<String, String>::new();
        let uri = p.url().clone();
        for cookie in p
            .headers()
            .get_all("Cookie")
            .into_iter()
            .map(|cookies| cookies.to_str().unwrap_or_default().to_string())
        {
            let splt = cookie
                .split('=')
                .into_iter()
                .map(String::from)
                .collect::<Vec<String>>();
            cookies.add(splt[0].clone(), Value::String(splt[1..].join("=")));
        }
        for (name, data) in p.headers().iter() {
            headers.insert(
                name.as_str().into(),
                SString::from(data.as_bytes()).unchecked_safe(),
            );
        }
        let status_code = p.status().as_u16();
        let (body, error) = match p.bytes() {
            Ok(bytes) => (bytes.to_vec(), None),
            Err(error) => (Vec::<u8>::new(), Some(error.to_string())),
        };
        HttpResponse {
            status_code,
            body: body,
            cookies: cookies,
            error: error,
            headers: headers,
            url: uri,
        }
    }
}

pub fn headers_to_json(headers: &HeaderMap) -> Result<BTreeMap<String, String>> {
    let mut result = BTreeMap::<String, String>::new();
    for (key, value) in headers.iter() {
        result.insert(
            key.to_string(),
            value.to_str().map(|v| v.to_string()).unwrap_or_default(),
        );
    }
    Ok(result)
}
