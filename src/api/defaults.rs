use reqwest::header::{HeaderMap, HeaderValue};
use std::sync::LazyLock;

pub const DEFAULT_BASE_URL: &'static str = "https://crates.io";
pub const DEFAULT_TIMEOUT_SECONDS: u64 = 10;

pub static USER_AGENT: LazyLock<String> = LazyLock::new(|| {
    let version = env!("CARGO_PKG_VERSION");
    format!("cargo-upgrade {version}")
});

pub fn default_headers() -> HeaderMap {
    let mut headers = HeaderMap::new();
    headers.insert("Accept", HeaderValue::from_static("application/json"));
    headers.insert("Content-Type", HeaderValue::from_static("application/json"));
    headers.insert("User-Agent", HeaderValue::from_str(&USER_AGENT).unwrap());
    headers
}
