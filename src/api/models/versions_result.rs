use crate::Result;

use reqwest::blocking::Response;
use serde::{Deserialize, Serialize};
use super::{FromResponse, EncodableVersion};

#[derive(Debug, Default, Deserialize, Serialize)]
pub struct VersionsResult {
    pub versions: Vec<EncodableVersion>,
}
impl FromResponse for VersionsResult {
    fn from_response(response: Response) -> Result<VersionsResult> {
        Ok(response.json::<VersionsResult>()?)
    }
}
