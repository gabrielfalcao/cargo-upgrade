use super::{EncodableError, EncodableVersion, FromResponse};
use serde::{Deserialize, Serialize};

#[derive(Debug, Default, Deserialize, Serialize)]
pub struct VersionsResult {
    pub versions: Vec<EncodableVersion>,

    pub errors: Option<EncodableError>,

    #[serde(flatten)]
    pub meta: serde_json::Value,
}
impl FromResponse for VersionsResult {}
