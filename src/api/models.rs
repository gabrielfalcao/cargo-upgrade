use crate::{Error, Result};
use crates_io_api_types::{EncodableCrate, EncodableVersion};

pub trait FromResponse {
    fn from_response(response: &Response) -> Result<Self>;
}

#[derive(Clone, Debug, Default, Deserialize, Serialize)]
pub struct SearchResult {
    pub crates: Vec<EncodableCrate>,
}
impl FromResponse for SearchResult {
    fn from_response(response: &Response) -> Result<SearchResult> {
        Ok(serde_json::from_str::<SearchResult>(
            &response.body().to_string(),
        ))
    }
}
#[derive(Clone, Debug, Default, Deserialize, Serialize)]
pub struct VersionsResult {
    pub versions: Vec<EncodableVersion>,
}
impl FromResponse for VersionsResult {
    fn from_response(response: &Response) -> Result<VersionsResult> {
        Ok(serde_json::from_str::<VersionsResult>(
            &response.body().to_string(),
        ))
    }
}
