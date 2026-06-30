use crate::{Error, Result};

use reqwest::{
    Url,
    blocking::{Request, Response},
};
use serde::{Deserialize, Serialize};
use chrono::{DateTime, Utc};

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct EncodablePublicUser {
    /// An opaque identifier for the user.
    pub id: i32,

    /// The user's login name.
    pub login: String,

    /// The user's display name, if set.
    pub name: Option<String>,

    /// The user's avatar URL, if set.
    pub avatar: Option<String>,

    /// The user's GitHub profile URL.
    pub url: String,

    /// The date and time the user was created.
    ///
    /// For users created before June 19, 2026, the creation time will be the
    /// time the user's GitHub account was created. If the GitHub account was
    /// deleted before June 19, 2026, this field will be empty.
    pub created_at: Option<DateTime<Utc>>,
}
