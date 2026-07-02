use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct EncodableVersionLinks {
    /// The API path to download this version's dependencies.
    pub dependencies: String,

    /// The API path to download this version's download numbers.
    pub version_downloads: String,

    /// The API path to download this version's authors.
    pub authors: String,
}
