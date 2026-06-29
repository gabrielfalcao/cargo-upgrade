use std::time::Duration;

use cargo_upgrade::cli::ParserDispatcher;
use cargo_upgrade::{Error, Manifest, Result, Api};
use crate::api::{
    edit_dependency_version, edit_edition_version, Api, DEFAULT_API_HOST,
    DEFAULT_USER_AGENT,
};
use crate::progress::{spinner, spinner_style, CargoTomlProgressHandler, DEFAULT_EDITION};
use crate::models::Manifest;

use clap::Parser;
use dumbeq::DumbEq;
use indicatif::{ProgressBar, ProgressStyle};
use iocore::{walk_dir, Path, WalkProgressHandler};
use toml_edit::{DocumentMut, Item, Value};

const DEFAULT_EDITION: &'static str = "2024";

#[derive(Parser, Debug)]
#[command(
    author,
    version,
    about,
    long_about = "cargo-upgrade command-line"
)]
pub struct Cli {
    #[arg()]
    pub packages: Vec<String>,

    #[arg(short, long)]
    pub input: Vec<Path>,

    #[arg(
        short,
        long,
        help = "(default) upgrades \"edition\" field to the value specified in --set-edition"
    )]
    pub edition: bool,

    #[arg(short, long, help = "do not modify \"edition\" field", conflicts_with_all=["edition", "set_edition"])]
    pub no_edition: bool,

    #[arg(
        short = 'N',
        long,
        help = "do not require latest dependency version to match semver"
    )]
    pub no_semver_required: bool,

    #[arg(
        long,
        requires = "edition",
        default_value = DEFAULT_EDITION.to_string(),
        help = "sets the \"edition\" field to this value"
    )]
    pub set_edition: Option<String>,

    #[arg(
        short,
        long,
        conflicts_with_all=["set_edition", "no_semver_required", "no_edition", "edition", "input"],
        help = "query crates but don't upgrade"
    )]
    pub query: Option<String>,

}
impl Cli {
    pub fn packages(&self, manifest_path: &Path) -> Vec<String> {
        (self.packages.len() > 0)
            .then(|| self.packages.clone())
            .unwrap_or_else(|| {
                self.all_packages(manifest_path)
                    .expect("trying to load names of dependencies from Cargo.toml")
            })
    }

    pub fn all_packages(&self, manifest_path: &Path) -> Result<Vec<String>> {
        Ok(Manifest::from_path(manifest_path)?.all_dependency_names())
    }

    pub fn set_edition(&self) -> String {
        self.set_edition
            .clone()
            .unwrap_or_else(|| DEFAULT_EDITION.to_string())
    }

    pub fn to_new_edition(&self) -> Option<String> {
        (!self.no_edition).then(|| self.set_edition())
    }

    pub fn paths(&self, pb: &ProgressBar) -> Result<Vec<Path>> {
        if self.input.is_empty() {
            Ok(walk_dir(&Path::cwd(), CargoTomlProgressHandler::new(pb), None)?)
        } else {
            Ok(self.input.clone())
        }
    }

    pub fn upgrade_manifest(
        &self,
        doc: DocumentMut,
        path: &Path,
        pb: &ProgressBar,
    ) -> Result<()> {
        pb.set_style(spinner_style(Some("{msg:.220}")));
        pb.set_message(format!("upgrading {:#?}", path.relative_to_cwd().to_string()));
        path.write(doc.to_string().as_bytes())?;
        Ok(())
    }

    pub fn get_newest_version(&self, package: &str) -> Result<String> {
        let (result, _) = self.search(package, 10)?;

        let result = result
            .iter()
            .filter(|package| {
                package
                    .max_version
                    .to_string()
                    .split(".")
                    .all(|part| part.chars().all(|c| c.is_ascii_alphabetic()))
            })
            .filter(|package| package.name.as_str() == package.name.as_str())
            .collect::<Vec<&Crate>>();

        if result.is_empty() {
            Err(Error::CratesIOError(format!("{} not found in crates.io", package)))
        } else if self.no_semver_required {
            Ok(result[0].max_version.to_string())
        } else {
            let mut index = 0usize;
            let mut errors = Vec::<String>::new();
            let latest_version = loop {
                if result.len() == 0 {
                    break result[0].name.to_string();
                } else {
                    let version = result[index].max_version.to_string();
                    let parts = version.split('.').enumerate().map(|(index, part)|(index, part, u32::from_str_radix(part, 10))).filter(|(index, part, result)|{
                        if let Some(error) = result.clone().err() {
                            eprintln!("invalid semver version ({part:#?}) at index {index} of {package} crate versions:  {error}");
                        }
                        result.is_ok()
                    }).map(|(_, _, result)|result.unwrap()).collect::<Vec<u32>>();
                    if parts.len() != 3 {
                        errors.push(format!(
                            "crate version {version:#?} does not respect semver"
                        ));
                        index += 1;
                    } else {
                        break version;
                    }
                }
            };
            Ok(latest_version)
        }
    }
    pub fn upgrade(&self) -> Result<()> {
        let pb = spinner(None);
        for path in self.paths(&pb)? {
            let manifest = path.read()?;
            let mut doc = manifest.parse::<DocumentMut>()?;

            if let Some(new_edition) = self.to_new_edition() {
                if let Some(old_edition) = edit_edition_version(&mut doc, &new_edition)
                {
                    if new_edition != old_edition {
                        self.upgrade_manifest(doc.clone(), &path, &pb)?;
                    }
                }
            }
            for package in self.packages(&path) {
                let newest_version = self.get_newest_version(package.as_str())?;

                for kind in [
                    "dependencies",
                    "dev-dependencies",
                    "build-dependencies",
                ] {
                    if let Some(old_version) = edit_dependency_version(
                        &mut doc,
                        kind,
                        package.as_str(),
                        &newest_version,
                    ) {
                        if old_version != newest_version {
                            println!(
                                "{}: upgraded {} from {:#?} to {:#?} in {}",
                                path.relative_to_cwd().to_string(),
                                package.as_str(),
                                &old_version,
                                &newest_version,
                                kind
                            );
                            self.upgrade_manifest(doc.clone(), &path, &pb)?;
                        }
                    }
                }
            }
        }
        Ok(())
    }
    pub fn query_only(&self, package: &str, max: usize) -> Result<()> {
        let mut handle = Easy::new();
        handle.useragent("cargo-upgrade (CLI)")?;
        let mut crates = Registry::new_handle(
            String::from("https://crates.io"),
            None,
            handle,
            false,
        );
        Ok((crates.search(package, max)?))
    }
    pub fn query_only(&self) -> Result<()> {
        Ok(())
    }
}

impl ParserDispatcher<Error> for Cli {
    fn dispatch(&self) -> Result<()> {
        if self.query {
            self.query_only()
        }
        else
        {
            self.upgrade();
        }
        Ok(())
    }
}

fn main() {
    Cli::main()
}
