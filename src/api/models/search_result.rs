
use super::{EncodableCrate, FromResponse};
use serde::{Deserialize, Serialize};

#[derive(Debug, Default, Deserialize, Serialize)]
pub struct SearchResult {
    pub crates: Vec<EncodableCrate>,

    #[serde(flatten)]
    pub meta: serde_json::Value,
}
impl FromResponse for SearchResult {}
