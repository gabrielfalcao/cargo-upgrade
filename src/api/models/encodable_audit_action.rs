use crate::{Error, Result};

use serde::{Deserialize, Serialize};
use chrono::{DateTime, Utc};
use super::EncodablePublicUser;

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct EncodableAuditAction {
    /// The action that was performed.
    #[schema(example = "publish")]
    pub action: String,

    /// The user who performed the action.
    pub user: EncodablePublicUser,

    /// The date and time the action was performed.
    #[schema(example = "2019-12-13T13:46:41Z")]
    pub time: DateTime<Utc>,
}
