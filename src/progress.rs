use std::time::Duration;

use crate::cli::ParserDispatcher;
use crate::{Error, Manifest, Result};
use clap::Parser;
use dumbeq::DumbEq;
use indicatif::{ProgressBar, ProgressStyle};
use iocore::{walk_dir, Path, WalkProgressHandler};
use toml_edit::{DocumentMut, Item, Value};

pub const DEFAULT_EDITION: &'static str = "2024";

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
            self.pb
                .set_message(format!("considering {filename}"));
        } else {
            self.pb.set_style(spinner_style(Some(
                "{spinner:.red} {msg:.242} {elapsed:.yellow}",
            )));
            self.pb.set_message(format!("working"));
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

pub fn spinner(template: Option<&str>) -> ProgressBar {
    let pb = ProgressBar::new_spinner();
    pb.enable_steady_tick(Duration::from_millis(37));
    pb.set_style(spinner_style(template));

    pb
}
pub fn spinner_style(template: Option<&str>) -> ProgressStyle {
    ProgressStyle::with_template(
        template.unwrap_or_else(|| "{spinner:.yellow} {msg:.cyan}"),
    )
    .unwrap()
    .tick_strings(&[
        "▐|\\____________▌",
        "▐_|\\___________▌",
        "▐__|\\__________▌",
        "▐___|\\_________▌",
        "▐____|\\________▌",
        "▐_____|\\_______▌",
        "▐______|\\______▌",
        "▐_______|\\_____▌",
        "▐________|\\____▌",
        "▐_________|\\___▌",
        "▐__________|\\__▌",
        "▐___________|\\_▌",
        "▐____________|\\▌",
        "▐____________/|▌",
        "▐___________/|_▌",
        "▐__________/|__▌",
        "▐_________/|___▌",
        "▐________/|____▌",
        "▐_______/|_____▌",
        "▐______/|______▌",
        "▐_____/|_______▌",
        "▐____/|________▌",
        "▐___/|_________▌",
        "▐__/|__________▌",
        "▐_/|___________▌",
        "▐/|____________▌",
    ])
}
