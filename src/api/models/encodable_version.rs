use crate::{Error, Result};

use reqwest::{
    Url,
    blocking::{Request, Response},
};
use serde::{Deserialize, Serialize};
use chrono::{DateTime, Utc};

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[schema(as = Version)]
pub struct EncodableVersion {
    /// An opaque identifier for the version.
    #[schema(example = 42)]
    pub id: i32,

    /// The name of the crate.
    #[serde(rename = "crate")]
    #[schema(example = "serde")]
    pub krate: String,

    /// The version number.
    #[schema(example = "1.0.0")]
    pub num: String,

    /// The API path to download the crate.
    #[schema(example = "/api/v1/crates/serde/1.0.0/download")]
    pub dl_path: String,

    /// The API path to download the crate's README file as HTML code.
    #[schema(example = "/api/v1/crates/serde/1.0.0/readme")]
    pub readme_path: String,

    /// The date and time this version was last updated (i.e. yanked or unyanked).
    #[schema(example = "2019-12-13T13:46:41Z")]
    pub updated_at: DateTime<Utc>,

    /// The date and time this version was created.
    #[schema(example = "2019-12-13T13:46:41Z")]
    pub created_at: DateTime<Utc>,

    /// The total number of downloads for this version.
    #[schema(example = 123_456)]
    pub downloads: i32,

    /// The features defined by this version.
    #[schema(value_type = BTreeMap<String, Vec<String>>)]
    pub features: serde_json::Value,

    /// Whether this version has been yanked.
    #[schema(example = false)]
    pub yanked: bool,

    /// The message given when this version was yanked, if any.
    #[schema(example = "Security vulnerability")]
    pub yank_message: Option<String>,

    /// The name of the native library this version links with, if any.
    #[schema(example = "git2")]
    pub lib_links: Option<String>,

    /// The license of this version of the crate.
    #[schema(example = "MIT")]
    pub license: Option<String>,

    /// Links to other API endpoints related to this version.
    pub links: EncodableVersionLinks,

    /// The size of the compressed crate file in bytes.
    #[schema(example = 1_234)]
    pub crate_size: i32,

    /// The user who published this version.
    ///
    /// This field may be `null` if the version was published before crates.io
    /// started recording this information.
    pub published_by: Option<EncodablePublicUser>,

    /// A list of actions performed on this version.
    #[schema(inline)]
    pub audit_actions: Vec<EncodableAuditAction>,

    /// The SHA256 checksum of the compressed crate file encoded as a
    /// hexadecimal string.
    #[schema(example = "e8dfc9d19bdbf6d17e22319da49161d5d0108e4188e8b680aef6299eed22df60")]
    pub checksum: String,

    /// The minimum version of the Rust compiler required to compile
    /// this version, if set.
    #[schema(example = "1.31")]
    pub rust_version: Option<String>,

    /// Whether this version can be used as a library.
    #[schema(example = true)]
    pub has_lib: Option<bool>,

    /// The names of the binaries provided by this version, if any.
    #[schema(example = json!([]))]
    pub bin_names: Option<Vec<Option<String>>>,

    /// The Rust Edition used to compile this version, if set.
    #[schema(example = "2021")]
    pub edition: Option<String>,

    /// The description of this version of the crate.
    #[schema(example = "A generic serialization/deserialization framework")]
    pub description: Option<String>,

    /// The URL to the crate's homepage, if set.
    #[schema(example = "https://serde.rs")]
    pub homepage: Option<String>,

    /// The URL to the crate's documentation, if set.
    #[schema(example = "https://docs.rs/serde")]
    pub documentation: Option<String>,

    /// The URL to the crate's repository, if set.
    #[schema(example = "https://github.com/serde-rs/serde")]
    pub repository: Option<String>,

    /// Information about the trusted publisher that published this version, if any.
    ///
    /// Status: **Unstable**
    ///
    /// This field is filled if the version was published via trusted publishing
    /// (e.g., GitHub Actions) rather than a regular API token.
    ///
    /// The exact structure of this field depends on the `provider` field
    /// inside it.
    #[schema(value_type = Option<HashMap<String, serde_json::Value>>)]
    pub trustpub_data: Option<TrustpubData>,

    /// Line count statistics for this version.
    ///
    /// Status: **Unstable**
    ///
    /// This field may be `null` until the version has been analyzed, which
    /// happens in an asynchronous background job.
    #[schema(value_type = Option<HashMap<String, serde_json::Value>>)]
    pub linecounts: Option<serde_json::Value>,
}
