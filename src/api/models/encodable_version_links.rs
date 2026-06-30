use crate::{Error, Result};

use reqwest::{
    Url,
    blocking::{Request, Response},
};
use serde::{Deserialize, Serialize};
use chrono::{DateTime, Utc};

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[schema(as = VersionLinks)]
pub struct EncodableVersionLinks {
    /// The API path to download this version's dependencies.
    #[schema(example = "/api/v1/crates/serde/1.0.0/dependencies")]
    pub dependencies: String,

    /// The API path to download this version's download numbers.
    #[schema(example = "/api/v1/crates/serde/1.0.0/downloads")]
    pub version_downloads: String,

    /// The API path to download this version's authors.
    #[schema(deprecated, example = "/api/v1/crates/serde/1.0.0/authors")]
    pub authors: String,
}
