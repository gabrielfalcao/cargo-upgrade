
use super::EncodableCrateLinks;
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct EncodableCrate {
    /// An opaque identifier for the crate.
    pub id: String,

    /// The name of the crate.
    pub name: String,

    /// The date and time this crate was last updated.
    pub updated_at: DateTime<Utc>,

    /// The list of version IDs belonging to this crate.
    pub versions: Option<Vec<i32>>,

    /// The list of keywords belonging to this crate.
    pub keywords: Option<Vec<String>>,

    /// The list of categories belonging to this crate.
    pub categories: Option<Vec<String>>,

    pub badges: [(); 0],

    /// The date and time this crate was created.
    pub created_at: DateTime<Utc>,

    /// The total number of downloads for this crate.
    pub downloads: i64,

    /// The total number of downloads for this crate in the last 90 days.
    pub recent_downloads: Option<i64>,

    /// The "default" version of this crate.
    ///
    /// This version will be displayed by default on the crate's page.
    pub default_version: Option<String>,

    /// The total number of versions for this crate.
    pub num_versions: i32,

    /// Whether all versions of this crate have been yanked.
    pub yanked: bool,

    /// The highest version number for this crate.
    pub max_version: String,

    /// The most recently published version for this crate.
    pub newest_version: String,

    /// The highest version number for this crate that is not a pre-release.
    pub max_stable_version: Option<String>,

    /// Description of the crate.
    pub description: Option<String>,

    /// The URL to the crate's homepage, if set.
    pub homepage: Option<String>,

    /// The URL to the crate's documentation, if set.
    pub documentation: Option<String>,

    /// The URL to the crate's repository, if set.
    pub repository: Option<String>,

    /// Links to other API endpoints related to this crate.
    pub links: EncodableCrateLinks,

    /// Whether the crate name was an exact match.
    pub exact_match: bool,

    /// Whether this crate can only be published via Trusted Publishing.
    pub trustpub_only: bool,
}
