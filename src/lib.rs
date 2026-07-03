mod errors;

pub use errors::{Error, Result};
pub mod cli;
pub use cli::ParserDispatcher;

pub(crate) mod models;
pub use models::{CookieJar, HttpRequest, HttpResponse, HttpVersion, Manifest};

pub mod api;
pub use api::{
    APIClient, DEFAULT_BASE_URL, DEFAULT_TIMEOUT_SECONDS, FromResponse, SearchResult,
    VersionsResult, default_headers, matches_semver, parse_semver,
};

pub(crate) mod util;
pub use util::{slugify, store_request, store_response};
