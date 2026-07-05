use super::{EncodableCrate, EncodableError, FromResponse};
use serde::{Deserialize, Serialize};

#[derive(Debug, Default, Deserialize, Serialize)]
pub struct SearchResult {
    pub crates: Vec<EncodableCrate>,

    pub errors: Option<EncodableError>,

    #[serde(flatten)]
    pub meta: serde_json::Value,
}
impl FromResponse for SearchResult {}
