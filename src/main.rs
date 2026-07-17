use std::time::Duration;

use cargo_upgrade::{
    APIClient, Error, Manifest, Result,
    api::models::{EncodableCrate, EncodableVersion},
    cli::ParserDispatcher,
    matches_semver, setup_logger,
};
use clap::Parser;
use dumbeq::DumbEq;
use indicatif::{ProgressBar, ProgressStyle};
use iocore::{Path, WalkProgressHandler, walk_dir};
use toml_edit::{DocumentMut, Item, Value};

const DEFAULT_EDITION: &'static str = "2024";

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = "cargo-upgrade command-line")]
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

    #[arg(short = 'N', long, help = "do not modify \"edition\" field", conflicts_with_all=["edition", "set_edition"])]
    pub no_edition: bool,

    #[arg(
        long,
        requires = "edition",
        help = "upgrades \"edition\" field if needed"
    )]
    pub set_edition: Option<String>,

    #[arg(short, long, help = "do not restrict versions to semantic versioning")]
    pub no_semver_filtering: bool,

    #[arg(short, long, help = "do not modify any files")]
    pub dry_run: bool,

    #[arg(short, long, help = "the log level")]
    pub log_level: Option<log::LevelFilter>,
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
            Ok(walk_dir(
                &Path::cwd(),
                CargoTomlProgressHandler::new(pb),
                None,
            )?)
        } else {
            Ok(self.input.clone())
        }
    }

    pub fn upgrade(&self, doc: DocumentMut, path: &Path, pb: &ProgressBar) -> Result<()> {
        pb.set_style(spinner_style(Some("{msg}\x1b[0m")));
        if self.dry_run {
            pb.set_message(format!(
                "\x1b[1;38;2;211;56;229mwould modify \x1b[1;38;2;255;159;206m{:#?}",
                path.relative_to_cwd().to_string()
            ));
        } else {
            pb.set_message(format!(
                "\x1b[1;38;2;141;204;48mupgrading \x1b[1;38;2;144;225;98m{:#?}",
                path.relative_to_cwd().to_string()
            ));
            path.write(doc.to_string().as_bytes())?;
        }
        Ok(())
    }

    pub fn search_package(&self, client: &APIClient, package_name: &str) -> Result<EncodableCrate> {
        let search_result = client.search_crate(package_name)?;

        for package in search_result.crates.into_iter() {
            if package.name == package_name {
                return Ok(package);
            }
        }
        Err(Error::CratesIOError(format!(
            "crate {package_name:#?} not found in crates.io",
        )))
    }
    pub fn get_package_versions(
        &self,
        client: &APIClient,
        package_name: &str,
    ) -> Result<Vec<EncodableVersion>> {
        let versions_result = client.get_crate_versions(package_name)?;
        Ok(versions_result.versions.clone())
    }

    pub fn get_newest_version(&self, package_name: &str) -> Result<EncodableVersion> {
        let client = APIClient::default();
        let _package = self.search_package(&client, package_name)?;
        let versions = self.get_package_versions(&client, package_name)?;

        let mut failed_to_semver = Vec::<String>::new();
        for version in versions.clone().into_iter() {
            if self.no_semver_filtering {
                return Ok(version);
            } else if matches_semver(&version.num) {
                return Ok(version);
            } else {
                failed_to_semver.push(version.num.to_string());
            }
        }
        let version_numbers = versions
            .iter()
            .map(|version| version.num.to_string())
            .collect::<Vec<String>>();
        let suffix = if failed_to_semver.len() > 0 {
            format!(
                "\nThe following versions were not considered due to not complying with semantic versioning: {}",
                failed_to_semver.join(", ")
            )
        } else {
            String::new()
        };
        Err(Error::CratesIOError(format!(
            "cannot find semver-compliant version for crate {package_name:#?} among available versions: {available_versions}.{suffix}",
            available_versions = version_numbers.join(", ")
        )))
    }
}

impl ParserDispatcher<Error> for Cli {
    fn dispatch(&self) -> Result<()> {
        setup_logger(self.log_level.unwrap_or_else(|| log::LevelFilter::Info));
        color_eyre::install()?;

        let pb = spinner(None);
        for path in self.paths(&pb)? {
            let manifest = path.read()?;
            let mut doc = manifest.parse::<DocumentMut>()?;

            if let Some(new_edition) = self.to_new_edition() {
                if let Some(old_edition) = edit_edition_version(&mut doc, &new_edition) {
                    if new_edition != old_edition {
                        self.upgrade(doc.clone(), &path, &pb)?;
                    }
                }
            }
            for package in self.packages(&path) {
                let newest_version = self.get_newest_version(package.as_str())?;

                for kind in ["dependencies", "dev-dependencies", "build-dependencies"] {
                    if let Some(old_version) = edit_dependency_version(
                        &mut doc,
                        kind,
                        package.as_str(),
                        &newest_version.num,
                    ) {
                        if old_version != newest_version.num {
                            println!(
                                "\x1b[1;38;2;238;234;89m{}: \x1b[1;38;2;178;231;126mupgraded \x1b[1;38;2;85;217;77m{} \x1b[1;38;2;178;231;126mfrom \x1b[1;38;2;238;234;89m{:#?} \x1b[1;38;2;178;231;126mto \x1b[1;38;2;85;217;77m{:#?} \x1b[1;38;2;178;231;126min \x1b[1;38;2;119;162;246m{}",
                                path.relative_to_cwd().to_string(),
                                package.as_str(),
                                old_version.to_string(),
                                newest_version.to_string(),
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
            }
            Some(Item::Value(Value::InlineTable(data))) => match data.get("version") {
                Some(Value::String(old_version)) => {
                    let old_version = old_version.clone().into_value().to_string();
                    doc[kind][package]["version"] = version.to_string().into();
                    return Some(old_version);
                }
                _ => {}
            },
            _ => {}
        },
        _ => {}
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
            }
            _ => None,
        },
        _ => None,
    }
}
fn main() -> std::result::Result<(), color_eyre::Report> {
    Cli::main()?;
    Ok(())
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
        Ok(path.name() == "Cargo.toml")
    }

    fn progress_in(&mut self, path: &Path, _: usize) -> iocore::Result<()> {
        let is_manifest = path.name() == "Cargo.toml";
        let filename = path.relative_to_cwd().to_string();
        if let Some(extension) = path.extension() {
            if extension.ends_with("toml") {
                self.pb
                    .set_style(spinner_style(Some("{spinner:.yellow} {msg:.cyan}")));
            }
        }
        if is_manifest {
            self.pb.set_message(format!("considering {filename}"));
        } else {
            self.pb.set_style(spinner_style(Some(
                "\x1b[1;38;2;144;225;98m{msg} \x1b[1;38;2;238;240;118m{spinner} \x1b[1;38;2;148;213;255m{elapsed}\x1b[0m",
            )));
            self.pb.set_message(format!("fetching"));
        }
        Ok(())
    }

    fn progress_out(&mut self, path: &Path) -> iocore::Result<()> {
        self.pb.set_style(spinner_style(Some(
            "{spinner:.cyan} {msg:.242} {elapsed:.blue}",
        )));
        self.pb.set_message(format!("finishing: {path}"));
        Ok(())
    }

    fn should_scan_directory(&mut self, path: &Path) -> iocore::Result<bool> {
        Ok(![".git", ".hg", "node_modules", "target"]
            .iter()
            .map(|h| h.to_string())
            .collect::<Vec<String>>()
            .contains(&path.name()))
    }
}

fn spinner(template: Option<&str>) -> ProgressBar {
    let pb = ProgressBar::new_spinner();
    pb.enable_steady_tick(Duration::from_millis(37));
    pb.set_style(spinner_style(template));

    pb
}
fn spinner_style(template: Option<&str>) -> ProgressStyle {
    ProgressStyle::with_template(template.unwrap_or_else(|| "{spinner:.yellow} {msg:.cyan}"))
        .unwrap()
        .tick_strings(&[
            "▰▱▱▱▱▱▱",
            "▰▱▱▱▱▱▱",
            "▰▰▱▱▱▱▱",
            "▰▰▱▱▱▱▱",
            "▰▰▰▱▱▱▱",
            "▰▰▰▱▱▱▱",
            "▰▰▰▰▱▱▱",
            "▰▰▰▰▱▱▱",
            "▰▰▰▰▰▱▱",
            "▰▰▰▰▰▱▱",
            "▰▰▰▰▰▰▱",
            "▰▰▰▰▰▰▱",
            "▰▰▰▰▰▰▰",
            "▰▰▰▰▰▰▰",
            "▰▰▰▰▰▰▱",
            "▰▰▰▰▰▰▱",
            "▰▰▰▰▰▱▱",
            "▰▰▰▰▰▱▱",
            "▰▰▰▰▱▱▱",
            "▰▰▰▰▱▱▱",
            "▰▰▰▱▱▱▱",
            "▰▰▰▱▱▱▱",
            "▰▰▱▱▱▱▱",
            "▰▰▱▱▱▱▱",
            "▰▱▱▱▱▱▱",
            "▰▱▱▱▱▱▱",
        ])
}
