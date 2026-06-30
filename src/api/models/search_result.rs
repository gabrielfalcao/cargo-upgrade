use crate::{Error, Result};

use reqwest::{
    Url,
    blocking::{Request, Response},
};
use serde::{Deserialize, Serialize};
use chrono::{DateTime, Utc};
use super::FromResponse;

#[derive(Debug, Default, Deserialize, Serialize)]
pub struct SearchResult {
    pub crates: Vec<EncodableCrate>,
}
impl FromResponse for SearchResult {
    fn from_response(response: Response) -> Result<SearchResult> {
        Ok(response.json::<SearchResult>()?)
    }
}
