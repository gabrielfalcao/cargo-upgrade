use super::{EncodableAuditAction, EncodablePublicUser, EncodableVersionLinks, TrustpubData};
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct EncodableVersion {
    /// An opaque identifier for the version.
    pub id: i32,

    /// The name of the crate.
    #[serde(rename = "crate")]
    pub krate: String,

    /// The version number.
    pub num: String,

    /// The API path to download the crate.
    pub dl_path: String,

    /// The API path to download the crate's README file as HTML code.
    pub readme_path: String,

    /// The date and time this version was last updated (i.e. yanked or unyanked).
    pub updated_at: DateTime<Utc>,

    /// The date and time this version was created.
    pub created_at: DateTime<Utc>,

    /// The total number of downloads for this version.
    pub downloads: i32,

    /// The features defined by this version.
    pub features: serde_json::Value,

    /// Whether this version has been yanked.
    pub yanked: bool,

    /// The message given when this version was yanked, if any.
    pub yank_message: Option<String>,

    /// The name of the native library this version links with, if any.
    pub lib_links: Option<String>,

    /// The license of this version of the crate.
    pub license: Option<String>,

    /// Links to other API endpoints related to this version.
    pub links: EncodableVersionLinks,

    /// The size of the compressed crate file in bytes.
    pub crate_size: i32,

    /// The user who published this version.
    ///
    /// This field may be `null` if the version was published before crates.io
    /// started recording this information.
    pub published_by: Option<EncodablePublicUser>,

    /// A list of actions performed on this version.
    pub audit_actions: Vec<EncodableAuditAction>,

    /// The SHA256 checksum of the compressed crate file encoded as a
    /// hexadecimal string.
    pub checksum: String,

    /// The minimum version of the Rust compiler required to compile
    /// this version, if set.
    pub rust_version: Option<String>,

    /// Whether this version can be used as a library.
    pub has_lib: Option<bool>,

    /// The names of the binaries provided by this version, if any.
    pub bin_names: Option<Vec<Option<String>>>,

    /// The Rust Edition used to compile this version, if set.
    pub edition: Option<String>,

    /// The description of this version of the crate.
    pub description: Option<String>,

    /// The URL to the crate's homepage, if set.
    pub homepage: Option<String>,

    /// The URL to the crate's documentation, if set.
    pub documentation: Option<String>,

    /// The URL to the crate's repository, if set.
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
    pub trustpub_data: Option<TrustpubData>,

    /// Line count statistics for this version.
    ///
    /// Status: **Unstable**
    ///
    /// This field may be `null` until the version has been analyzed, which
    /// happens in an asynchronous background job.
    pub linecounts: Option<serde_json::Value>,
}

impl std::fmt::Display for EncodableVersion {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(
            f,
            "{version}",
            version=self.num.to_string(),
        )
    }
}
