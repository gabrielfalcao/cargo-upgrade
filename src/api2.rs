use crate::{Error, Result};
use http::{Method, Request, Response, StatusCode};
use percent_encoding::{NON_ALPHANUMERIC, percent_encode};
use serde_json::Value;
use url::Url;

const DEFAULT_BASE_URL: &'static str = "https://crates.io/api/v1";
#[derive(Clone, Debug, Default)]
pub struct Api {
    base_url: Option<Url>,
}

impl Api {
    pub fn new(base_url: Option<Url>) -> Api {
        Api { base_url }
    }

    fn base_url(&self) -> Url {}
    fn request(&self, method: Method, url: Url) -> Response {}
    pub fn versions(&self, package_name: &str) -> Result<VersionsResult> {
        let response = self.request(
            Method::GET,
            Url::parse(format!(
                "/crates/{package_name}/versions?sort=date&per_page=100&include=release_tracks",
                package_name = package_name
            ))
            .unwrap(),
        )?;
        Ok(VersionsResult::from_response(response))
    }
    pub fn search(&self, package_name: &str) -> Result<SearchResult> {
        let response = self.request(
            Method::GET,
            Url::parse(format!(
                "/crates?q={package_name}&per_page=10&sort=relevance",
                package_name = percent_encode(package_name.as_bytes(), NON_ALPHANUMERIC)
            ))
            .unwrap(),
        )?;
        Ok(SearchResult::from_response(response))
    }
}

#[derive(Clone, Debug, Default)]
pub struct SearchResultItem {
    id: String,                 // "http"
    name: String,               // "http"
    updated_at: String,         // "2026-06-08T13:35:47.802375Z"
    versions: String,           // null
    keywords: String,           // null
    categories: String,         // null
    badges: String,             // []
    created_at: String,         // "2014-11-20T23:30:38.809367Z"
    downloads: String,          // 802608172
    recent_downloads: String,   // 189132700
    default_version: String,    // "1.4.2"
    num_versions: String,       // 44
    yanked: String,             // false
    max_version: String,        // "1.4.2"
    newest_version: String,     // "1.4.2"
    max_stable_version: String, // "1.4.2"
    description: String,        // "A set of types for representing HTTP requests and responses.\n"
    homepage: String,           // null
    documentation: String,      // "https://docs.rs/http"
    repository: String,         // "https://github.com/hyperium/http"
    links: ApiResponseLinks,
    exact_match: String, // true
    trustpub_only: bool,
}
#[derive(Clone, Debug, Default)]
pub struct ApiResponseLinks {
    links: BTreeMap<String, String>,
}

#[derive(Clone, Debug, Default)]
pub struct SearchResult {}
