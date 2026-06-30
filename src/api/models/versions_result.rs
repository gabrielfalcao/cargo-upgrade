use crate::{Error, Result};

use reqwest::{
    Url,
    blocking::{Request, Response},
};
use serde::{Deserialize, Serialize};
use chrono::{DateTime, Utc};
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
