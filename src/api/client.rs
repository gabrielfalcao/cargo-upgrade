use crate::{
    Error, HttpRequest, HttpResponse, Result,
    api::{
        defaults::{DEFAULT_BASE_URL, DEFAULT_TIMEOUT_SECONDS, default_headers},
        models::{FromResponse, SearchResult, VersionsResult},
    },
    headers_to_json, store_request,
};
use iocore::Path;
use percent_encoding::{NON_ALPHANUMERIC, percent_encode};
use reqwest::{
    Method,
    blocking::{Client, ClientBuilder, Response},
};
use std::{string::ToString, time::Duration};
use url::Url;

#[derive(Clone, Debug, Default)]
pub struct APIClient {
    base_url: Option<Url>,
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
            client,
        }
    }
    fn base_url(&self) -> Url {
        self.base_url
            .clone()
            .unwrap_or_else(|| Url::parse(DEFAULT_BASE_URL).unwrap())
    }
    fn request<T: ToString>(&self, method: Method, path: T) -> Result<Response> {
        let path = path.to_string();
        let full_url = self.base_url().join(&path)?;
        let request = self.client.request(method, full_url).build()?;
        let headers = request.headers().clone();
        let serde_request = HttpRequest::from(&request);
        // eprintln!(
        //     "json_headers: {json}",
        //     json = serde_json::to_string_pretty(&headers_to_json(&headers)?)?
        // );
        let (json_path, json_string) = serde_request.info().save(None)?;
        eprintln!(
            "json_request: {json_string}",
        );
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
        let result = VersionsResult::parse(response)?;
        dbg!(&result);
        Ok(result)
    }
    pub fn search_crate(&self, package_name: &str) -> Result<SearchResult> {
        let response = self.request(
            Method::GET,
            format!(
                "/api/v1/crates?q={package_name}&per_page=10&sort=relevance",
                package_name = percent_encode(package_name.as_bytes(), NON_ALPHANUMERIC)
            ),
        )?;
        let response = HttpResponse::from(response);
        let bytes = response.bytes()?.to_vec();
        let json_value = serde_json::from_str::<serde_json::Value>(&response.text()?)?;
        match SearchResult::from_json_bytes(bytes) {
            Ok(result) => Ok(result),
            Err(error) => {
                eprintln!("failed to search crate {package_name:#?}: {error}");
                eprintln!("response: {json_value:#?}");

                Err(Error::ParseError(error.to_string()))
            }
        }
        // dbg!(&result);
        // Ok(result)
    }
}
