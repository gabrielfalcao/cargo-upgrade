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
    #[schema(example = 42)]
    pub id: i32,

    /// The user's login name.
    #[schema(example = "ghost")]
    pub login: String,

    /// The user's display name, if set.
    #[schema(example = "Kate Morgan")]
    pub name: Option<String>,

    /// The user's avatar URL, if set.
    #[schema(example = "https://avatars2.githubusercontent.com/u/1234567?v=4")]
    pub avatar: Option<String>,

    /// The user's GitHub profile URL.
    #[schema(example = "https://github.com/ghost")]
    pub url: String,

    /// The date and time the user was created.
    ///
    /// For users created before June 19, 2026, the creation time will be the
    /// time the user's GitHub account was created. If the GitHub account was
    /// deleted before June 19, 2026, this field will be empty.
    pub created_at: Option<DateTime<Utc>>,
}
