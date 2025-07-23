mod errors;
use std::collections::BTreeMap;

pub use errors::{Error, Result};
pub mod cli;
pub use cli::ParserDispatcher;
use iocore::Path;
use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize, Clone, Debug, PartialEq)]
pub struct Manifest {
    pub dependencies: Option<BTreeMap<String, toml::Value>>,

    #[serde(rename(deserialize = "dev-dependencies"))]
    pub dev_dependencies: Option<BTreeMap<String, toml::Value>>,

    #[serde(rename(deserialize = "build-dependencies"))]
    pub build_dependencies: Option<BTreeMap<String, toml::Value>>,

    #[serde(flatten)]
    pub meta: toml::Value, // #[serde(from = "toml::Value")]
}

impl Manifest {
    pub fn from_path(path: &Path) -> Result<Manifest> {
        Ok(toml::from_str::<Manifest>(&path.read()?)?)
    }

    pub fn all_dependency_names(&self) -> Vec<String> {
        let mut deps = Vec::<String>::new();
        for dependencies in [
            &self.dependencies,
            &self.dev_dependencies,
            &self.build_dependencies,
        ]
        .into_iter()
        .filter(|deps| deps.is_some())
        .map(|deps| deps.clone().unwrap())
        {
            deps.extend(
                dependencies
                    .keys()
                    .map(String::from)
                    .collect::<Vec<String>>(),
            );
        }
        deps
    }
}
