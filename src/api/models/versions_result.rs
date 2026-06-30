use crate::Result;

use super::{EncodableVersion, FromResponse};
use reqwest::blocking::Response;
use serde::{Deserialize, Serialize};

#[derive(Debug, Default, Deserialize, Serialize)]
pub struct VersionsResult {
    pub versions: Vec<EncodableVersion>,
}
impl FromResponse for VersionsResult {}
