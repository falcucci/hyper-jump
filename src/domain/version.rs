use anyhow::Result;
use regex::Regex;
use semver::Version;
use serde::Deserialize;
use serde::Serialize;

/// Represents a local version of the software.
#[derive(Clone, PartialEq, Debug)]
pub struct LocalVersion {
    pub file_name: String,
    pub file_format: String,
    pub path: String,
    pub semver: Option<Version>,
}

/// Represents a remote version retrieved from GitHub.
#[derive(Serialize, Deserialize, Debug, PartialEq, Eq, Clone)]
pub struct RemoteVersion {
    pub name: String,
    pub tag_name: String,
    pub prerelease: bool,
}

#[derive(Debug, Clone)]
pub enum VersionStatus {
    Installed,
    Used,
    NotInstalled,
}

/// Represents a parsed version of the software.
#[derive(Debug, Clone)]
pub struct ParsedVersion {
    pub tag_name: String,
    pub non_parsed_string: String,
    pub semver: Option<Version>,
}

pub async fn parse_normal_version(version: &str) -> Result<ParsedVersion> {
    let semver = semver(version)?;
    let returned_version = match (semver, version.starts_with('v')) {
        (true, false) => parse_semver(version)?,
        _ => ParsedVersion {
            tag_name: version.to_string(),
            non_parsed_string: version.to_string(),
            semver: None,
        },
    };

    Ok(returned_version)
}

pub fn semver(version: &str) -> Result<bool> {
    Ok(Regex::new(r"^v?[0-9]+\.[0-9]+\.[0-9]+$")?.is_match(version))
}

fn parse_semver(version: &str) -> Result<ParsedVersion> {
    let version = version.to_string();
    let semver = Version::parse(&version)?;
    Ok(ParsedVersion {
        tag_name: version.clone(),
        non_parsed_string: version.clone(),
        semver: Some(semver),
    })
}
