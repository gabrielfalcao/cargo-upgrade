use super::EncodablePublicUser;
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct EncodableAuditAction {
    /// The action that was performed.
    pub action: String,

    /// The user who performed the action.
    pub user: EncodablePublicUser,

    /// The date and time the action was performed.
    pub time: DateTime<Utc>,
}
