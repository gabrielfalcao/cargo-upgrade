use crate::{
    Error, Result,
    api::{
        defaults::{DEFAULT_BASE_URL, DEFAULT_TIMEOUT_SECONDS, default_headers},
        models::{FromResponse, SearchResult, VersionsResult},
    },
};
use crates_io_api_types::{EncodableCrate, EncodableVersion};

use percent_encoding::{NON_ALPHANUMERIC, percent_encode};
use reqwest::{
    Method, StatusCode, Url,
    blocking::{Client, ClientBuilder, Request, Response},
    header::{HeaderMap, HeaderValue},
};
use serde::{Deserialize, Serialize};
use serde_json::Value;
use std::{string::ToString, time::Duration};

#[derive(Clone, Debug, Default)]
pub struct APIClient {
    base_url: Option<Url>,
    timeout: Option<Duration>,
    client: Client,
}

impl APIClient {
    pub fn new(base_url: Option<Url>, timeout: Option<Duration>) -> APIClient {
        let timeout = timeout
            .clone()
            .unwrap_or_else(|| Duration::from_secs(DEFAULT_TIMEOUT_SECONDS));
        let base_url = base_url
            .clone()
            .unwrap_or_else(|| Url::parse(DEFAULT_BASE_URL).unwrap());
        let client = ClientBuilder::new()
            .timeout(timeout)
            .https_only(true)
            .default_headers(default_headers())
            .build()
            .unwrap();
        APIClient {
            base_url: Some(base_url),
            timeout: Some(timeout),
            client,
        }
    }
    fn default_headers(&self) -> HeaderMap {
        let mut headers = HeaderMap::new();
        headers.insert("User-Agent", HeaderValue::from_static("crates.io client"));
        headers
    }

    fn base_url(&self) -> Url {
        self.base_url
            .clone()
            .unwrap_or_else(|| Url::parse(DEFAULT_BASE_URL).unwrap())
    }
    fn timeout(&self) -> Duration {
        self.timeout
            .clone()
            .unwrap_or_else(|| Duration::from_secs(DEFAULT_TIMEOUT_SECONDS))
    }
    fn request<T: ToString>(&self, method: Method, path: T) -> Result<Response> {
        let path = path.to_string();
        let full_url = self.base_url().join(&path)?;
        let request = self.client.request(method, full_url).build()?;
        let response = self.client.execute(request)?;
        Ok(response)
    }
    pub fn get_crate_versions(&self, package_name: &str) -> Result<VersionsResult> {
        let response = self.request(
            Method::GET,
            format!(
                "/api/v1/crates/{package_name}/versions?sort=date&per_page=100&include=release_tracks",
                package_name = package_name
            )
        )?;
        Ok(VersionsResult::from_response(response)?)
    }
    pub fn search_crate(&self, package_name: &str) -> Result<SearchResult> {
        let response = self.request(
            Method::GET,
            format!(
                "/api/v1/crates?q={package_name}&per_page=10&sort=relevance",
                package_name = percent_encode(package_name.as_bytes(), NON_ALPHANUMERIC)
            ),
        )?;
        Ok(SearchResult::from_response(response)?)
    }
}
