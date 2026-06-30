
use serde::{Deserialize, Serialize};
use chrono::{DateTime, Utc};
use super::EncodablePublicUser;

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct EncodableAuditAction {
    /// The action that was performed.
    pub action: String,

    /// The user who performed the action.
    pub user: EncodablePublicUser,

    /// The date and time the action was performed.
    pub time: DateTime<Utc>,
}
