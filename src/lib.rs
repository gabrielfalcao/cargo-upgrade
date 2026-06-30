mod errors;
use std::collections::BTreeMap;

pub use errors::{Error, Result};
pub mod cli;
pub use cli::ParserDispatcher;
use iocore::Path;
use serde::{Deserialize, Serialize};

pub(crate) mod models;
pub use models::Manifest;

pub mod api;
pub use api::{
    APIClient, DEFAULT_BASE_URL, DEFAULT_TIMEOUT_SECONDS, FromResponse, SearchResult,
    VersionsResult, default_headers, matches_semver, parse_semver,
};
