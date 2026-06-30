use crate::Result;

use super::{EncodableCrate, FromResponse};
use reqwest::blocking::Response;
use serde::{Deserialize, Serialize};

#[derive(Debug, Default, Deserialize, Serialize)]
pub struct SearchResult {
    pub crates: Vec<EncodableCrate>,
}
impl FromResponse for SearchResult {
    fn from_response(response: Response) -> Result<SearchResult> {
        Ok(response.json::<SearchResult>()?)
    }
}
