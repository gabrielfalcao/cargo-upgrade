use crate::{Error, Result};

use super::{EncodableCrate, FromResponse};
use chrono::{DateTime, Utc};
use reqwest::{
    Url,
    blocking::{Request, Response},
};
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
