mod errors;
use std::collections::BTreeMap;

pub use errors::{Error, Result};
pub mod cli;
pub use cli::ParserDispatcher;

pub(crate) mod api;
pub(crate) mod progress;
pub(crate) mod models;

pub use api::{
    edit_dependency_version, edit_edition_version, Api, DEFAULT_API_HOST,
    DEFAULT_USER_AGENT,
};
pub use progress::{spinner, spinner_style, CargoTomlProgressHandler, DEFAULT_EDITION};
pub use models::Manifest;
