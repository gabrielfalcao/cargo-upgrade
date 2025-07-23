use std::time::Duration;

use cargo_upgrade::cli::ParserDispatcher;
use cargo_upgrade::{Error, Result};
use clap::Parser;
use crates_io::{Crate, Registry};
use curl::easy::Easy;
use dumbeq::DumbEq;
use indicatif::{ProgressBar, ProgressStyle};
use iocore::{walk_dir, Path, WalkProgressHandler};
use toml_edit::{DocumentMut, Item, Value};

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
}
impl Cli {
    pub fn paths(&self, pb: &ProgressBar) -> Result<Vec<Path>> {
        if self.input.is_empty() {
            Ok(walk_dir(
                &Path::cwd(),
                CargoTomlProgressHandler::new(pb),
                None,
            )?)
        } else {
            Ok(self.input.clone())
        }
    }

    pub fn upgrade(
        &self,
        doc: DocumentMut,
        path: &Path,
        pb: &ProgressBar,
    ) -> Result<()> {
        pb.set_message(format!(
            "upgrading {:#?}",
            path.relative_to_cwd().to_string()
        ));
        path.write(doc.to_string().as_bytes())?;
        Ok(())
    }

    pub fn get_newest_version(&self, package: &str) -> Result<String> {
        let mut handle = Easy::new();
        handle.useragent("cargo-upgrade (CLI)")?;
        let mut crates = Registry::new_handle(
            String::from("https://crates.io"),
            None,
            handle,
            false,
        );
        let (result, _) = crates.search(package, 10)?;

        let result = result
            .iter()
            .filter(|package| package.name.as_str() == package.name.as_str())
            .collect::<Vec<&Crate>>();

        if result.is_empty() {
            Err(Error::CratesIOError(format!(
                "{} not found in crates.io",
                package
            )))
        } else {
            Ok(result[0].max_version.to_string())
        }
    }
}

impl ParserDispatcher<Error> for Cli {
    fn dispatch(&self) -> Result<()> {
        let pb = spinner();
        for path in self.paths(&pb)? {
            let manifest = path.read()?;
            let mut doc = manifest.parse::<DocumentMut>()?;

            for package in self.packages.clone() {
                let newest_version =
                    self.get_newest_version(package.as_str())?;

                for kind in [
                    "dependencies",
                    "dev-dependencies",
                    "build-dependencies",
                ] {
                    if let Some(old_version) = edit_version(
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
                            self.upgrade(doc.clone(), &path, &pb)?;
                        }
                    }
                }
            }
        }
        Ok(())
    }
}

fn edit_version(
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
            Some(Item::Value(Value::InlineTable(data))) => match data
                .get("version")
            {
                Some(Value::String(old_version)) => {
                    let old_version =
                        old_version.clone().into_value().to_string();
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
fn main() {
    Cli::main()
}

#[derive(Clone, DumbEq)]
pub struct CargoTomlProgressHandler {
    pub pb: ProgressBar,
}
impl CargoTomlProgressHandler {
    pub fn new(pb: &ProgressBar) -> CargoTomlProgressHandler {
        CargoTomlProgressHandler { pb: pb.clone() }
    }
}
impl WalkProgressHandler for CargoTomlProgressHandler {
    fn path_matching(&mut self, path: &Path) -> iocore::Result<bool> {
        self.pb.set_message(format!(
            "scanning {:#?}",
            path.relative_to_cwd().to_string()
        ));
        let path = path.canonicalize()?;
        Ok(path.name() == "Cargo.toml" && path.is_file())
    }

    fn should_scan_directory(&mut self, path: &Path) -> iocore::Result<bool> {
        Ok(![".git", ".hg", "node_modules", "target"]
            .iter()
            .map(|h| h.to_string())
            .collect::<Vec<String>>()
            .contains(&path.name()))
    }
}

fn spinner() -> ProgressBar {
    let pb = ProgressBar::new_spinner();
    pb.enable_steady_tick(Duration::from_millis(37));
    pb.set_style(
        ProgressStyle::with_template("{spinner:.yellow} {msg:.cyan}")
            .unwrap()
            .tick_strings(&[
                "( ●    )",
                "(  ●   )",
                "(   ●  )",
                "(    ● )",
                "(     ●)",
                "(    ● )",
                "(   ●  )",
                "(  ●   )",
                "( ●    )",
                "(●     )",
            ]),
    );
    pb
}
