use std::fmt::Display;
use std::str::ToString;
use std::time::Duration;

use crates_io::{Crate, Registry};
use curl::easy::Easy;
use dumbeq::DumbEq;
use indicatif::{ProgressBar, ProgressStyle};
use iocore::{walk_dir, Path, WalkProgressHandler};
use toml_edit::{DocumentMut, Item, Value};

use crate::cli::ParserDispatcher;
use crate::{Error, Manifest, Result};

const DEFAULT_EDITION: &'static str = "2024";
pub const DEFAULT_USER_AGENT: &'static str = "cargo-upgrade (CLI)";
pub const DEFAULT_API_HOST: &'static str = "https://crates.io";

#[derive(Debug, Clone)]
#[command(
    author,
    version,
    about,
    long_about = "cargo-upgrade command-line"
)]
pub struct Api {
    pub registry: Registry,
    pub criteria: Vec<String>,
}
impl Api {
    pub fn new() -> Api {
        Ok(Api::new_with_options(
            Default::default(),
            Default::default(),
            Default::default(),
            Default::default(),
        )?)
    }

    pub fn new_with_options<
        T: ToString,
        U: ToString,
        H: ToString,
        S: Display,
        I: IntoIterator<Item = S>,
    >(
        api_host: Option<H>,
        token: Option<T>,
        user_agent: Option<U>,
        criteria: I,
    ) -> Api {
        let mut handle = Easy::new();
        match user_agent.map(ToString::to_string) {
            Some(ua) => handle.useragent(&ua)?,
            None => {},
        };

        let mut registry = Registry::new_handle(
            api_host.map(ToString::to_string),
            None,
            handle,
            false,
        );
        let criteria = criteria
            .into_iter()
            .map(ToString::to_string)
            .collect::<Vec<String>>();
        Ok(Api { registry, criteria })
    }

    pub fn search<T: Display>(&self, package: T, max: usize) -> Result<()> {
        let package = package.to_string();
        Ok((self.registry.search(&package, max)?))
    }
}

impl ParserDispatcher<Error> for Api {
    fn dispatch(&self) -> Result<()> {
        if self.query {
            self.query_only()
        } else {
            self.upgrade();
        }
        Ok(())
    }
}

fn edit_dependency_version(
    doc: &mut DocumentMut,
    kind: &str,
    package: &str,
    version: &str,
) -> Option<String> {
    match doc.get(kind) {
        Some(Item::Table(dependencies)) => match dependencies.get(package) {
            Some(Item::Value(Value::String(old_version))) => {
                let old_version = old_version.clone().into_value().to_string();
                doc[kind][package] = version.to_string().into();
                return Some(old_version);
            },
            Some(Item::Value(Value::InlineTable(data))) => match data.get("version") {
                Some(Value::String(old_version)) => {
                    let old_version = old_version.clone().into_value().to_string();
                    doc[kind][package]["version"] = version.to_string().into();
                    return Some(old_version);
                },
                _ => {},
            },
            _ => {},
        },
        _ => {},
    }
    None
}
fn edit_edition_version(doc: &mut DocumentMut, edition: &str) -> Option<String> {
    match doc.get("package") {
        Some(Item::Table(package)) => match package.get("edition") {
            Some(Item::Value(Value::String(old_edition))) => {
                let old_edition = old_edition.clone().into_value().to_string();
                doc["package"]["edition"] = edition.to_string().into();
                Some(old_edition)
            },
            _ => None,
        },
        _ => None,
    }
}
