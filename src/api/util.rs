use crate::{Error, Result};
use regex::Regex;
use std::sync::LazyLock;

static SEMVER_REGEX: LazyLock<Regex> = LazyLock::new(|| {
    Regex::new(r"^(?<major>[0-9]+)[.](?<minor>[0-9]+)[.](?<patch>[0-9]+)$").unwrap()
});

pub fn parse_semver(version: &str) -> Result<(u64, u64, u64)> {
    match SEMVER_REGEX.captures(version) {
        Some(caps) => {
            let major_str = caps.name("major").unwrap().as_str();
            let minor_str = caps.name("minor").unwrap().as_str();
            let patch_str = caps.name("patch").unwrap().as_str();

            let major = u64::from_str_radix(&major_str, 10)?;
            let minor = u64::from_str_radix(&minor_str, 10)?;
            let patch = u64::from_str_radix(&patch_str, 10)?;

            Ok((major, minor, patch))
        }
        None => Err(Error::ParseError(format!(
            "cannot parse semantic version from {version:#?}"
        ))),
    }
}

pub fn matches_semver(version: &str) -> bool {
    parse_semver(version)
        .map(|_| true)
        .unwrap_or_else(|_| false)
}
