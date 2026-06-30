use reqwest::header::{HeaderMap, HeaderValue};

pub const DEFAULT_BASE_URL: &'static str = "https://crates.io";
pub const DEFAULT_TIMEOUT_SECONDS: u64 = 10;

pub fn default_headers() -> HeaderMap {
    let mut headers = HeaderMap::new();
    headers.insert("Accept", HeaderValue::from_static("application/json"));
    headers.insert("Content-Type", HeaderValue::from_static("application/json"));
    headers.insert("User-Agent", HeaderValue::from_static("crates.io client"));
    headers
}
