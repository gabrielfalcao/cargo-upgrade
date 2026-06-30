
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct EncodableCrateLinks {
    /// The API path to this crate's download statistics.
    pub version_downloads: String,

    /// The API path to this crate's versions.
    pub versions: Option<String>,

    /// The API path to this crate's owners.
    pub owners: Option<String>,

    /// The API path to this crate's team owners.
    pub owner_team: Option<String>,

    /// The API path to this crate's user owners.
    pub owner_user: Option<String>,

    /// The API path to this crate's reverse dependencies.
    pub reverse_dependencies: String,
}
