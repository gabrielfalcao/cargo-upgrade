use crate::{Error, Result};
use crates_io_api_types::{EncodableCrate, EncodableVersion};
use reqwest::{
    Url,
    blocking::{Request, Response},
};
use serde::{Deserialize, Serialize};

pub trait FromResponse {
    fn from_response(response: Response) -> Result<Self>
    where
        Self: Sized;
}

#[derive(Debug, Default, Deserialize, Serialize)]
pub struct SearchResult {
    pub crates: Vec<EncodableCrate>,
}
impl FromResponse for SearchResult {
    fn from_response(response: Response) -> Result<SearchResult> {
        Ok(response.json::<SearchResult>()?)
    }
}
#[derive(Debug, Default, Deserialize, Serialize)]
pub struct VersionsResult {
    pub versions: Vec<EncodableVersion>,
}
impl FromResponse for VersionsResult {
    fn from_response(response: Response) -> Result<VersionsResult> {
        Ok(response.json::<VersionsResult>()?)
    }
}
