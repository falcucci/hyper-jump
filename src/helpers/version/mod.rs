use anyhow::anyhow;
use anyhow::Result;
use regex::Regex;
use semver::Version;
use tokio::fs;

use crate::commands::install::Package;

/// Represents a local version of the software.
///
/// This struct contains information about a local version of the software,
/// including the file name, file format, path, and semantic version.
///
/// # Fields
///
/// * `file_name: String` - The name of the file that contains the local
///   version.
/// * `file_format: String` - The format of the file that contains the local
///   version.
/// * `path: String` - The path to the file that contains the local version.
/// * `semver: Option<Version>` - The semantic version of the local version, or
///   `None` if the version is not a semantic version.
///
/// # Example
///
/// ```rust
/// let local_version = LocalVersion {
///     file_name: "version-1.0.0.tar.gz".to_string(),
///     file_format: "tar.gz".to_string(),
///     path: "/path/to/version-1.0.0.tar.gz".to_string(),
///     semver: Some(Version::parse("1.0.0").unwrap()),
/// };
/// println!("The local version is {:?}", local_version);
/// ```
#[derive(Clone, PartialEq)]
pub struct LocalVersion {
    pub file_name: String,
    pub file_format: String,
    pub path: String,
    pub semver: Option<Version>,
}

/// Represents a parsed version of the software.
///
/// This struct contains information about a parsed version of the software,
/// including the tag name, version type, non-parsed string, and semantic
/// version.
///
/// # Fields
///
/// * `tag_name: String` - The tag name of the parsed version.
/// * `version_type: VersionType` - The type of the parsed version.
/// * `non_parsed_string: String` - The non-parsed string of the parsed version.
/// * `semver: Option<Version>` - The semantic version of the parsed version, or
///   `None` if the version is not a semantic version.
///
/// # Example
///
/// ```rust
/// let parsed_version = ParsedVersion {
///     tag_name: "v1.0.0".to_string(),
///     version_type: VersionType::Normal,
///     non_parsed_string: "version-1.0.0".to_string(),
///     semver: Some(Version::parse("1.0.0").unwrap()),
/// };
/// println!("The parsed version is {:?}", parsed_version);
/// ```
#[derive(Debug, Clone)]
pub struct ParsedVersion {
    pub tag_name: String,
    pub version_type: VersionType,
    pub non_parsed_string: String,
    pub semver: Option<Version>,
}

/// Represents the type of a software version.
///
/// This enum is used to distinguish between different types of software
/// versions, such as normal versions, the latest version, nightly versions,
/// versions identified by a hash, and nightly versions that have been rolled
/// back.
///
/// # Variants
///
/// * `Normal` - Represents a normal version.
/// * `Latest` - Represents the latest version.
/// * `Hash` - Represents a version identified by a hash.
///
/// # Example
///
/// ```rust
/// let version_type = VersionType::Normal;
/// match version_type {
///     VersionType::Normal => println!("This is a normal version."),
///     VersionType::Latest => println!("This is the latest version."),
///     VersionType::Hash => println!("This is a version identified by a hash."),
/// }
/// ```
#[derive(PartialEq, Eq, Debug, Clone)]
pub enum VersionType {
    Normal,
    Latest,
    Hash,
}

pub async fn parse_version_type(version: &str) -> Result<ParsedVersion> {
    let version_regex = Regex::new(r"^v?[0-9]+\.[0-9]+\.[0-9]+$").unwrap();
    if version_regex.is_match(version) {
        let mut returned_version = version.to_string();
        if !version.contains('v') {
            returned_version.insert(0, 'v');
        }

        return Ok(ParsedVersion {
            tag_name: returned_version,
            version_type: VersionType::Normal,
            non_parsed_string: version.to_string(),
            semver: None,
        });
    }

    Err(anyhow!("Please provide a proper version string"))
}

/// This function reads the downloads directory and checks if there is a
/// directory with the name matching the specified version. If such a directory
/// is found, it means that the version is installed.
///
/// # Arguments
///
/// * `version` - The version to check.
///
/// # Returns
///
/// * `Result<bool>` - Returns a `Result` that contains `true` if the version is
///   installed, `false` otherwise, or an error if the operation failed.
///
/// # Errors
///
/// This function will return an error if:
///
/// * The downloads directory cannot be retrieved.
/// * The downloads directory cannot be read.
///
/// # Example
///
/// ```rust
/// let version = "1.0.0";
/// let is_installed = is_version_installed(version).await.unwrap();
/// println!("Is version {} installed? {}", version, is_installed);
/// ```
pub async fn is_version_installed(version: &str, package: Package) -> Result<bool> {
    let downloads_dir = crate::fs::get_downloads_directory(package).await?;
    let mut dir = tokio::fs::read_dir(&downloads_dir).await?;

    while let Some(directory) = dir.next_entry().await? {
        let name = directory.file_name().to_str().unwrap().to_owned();
        if !version.eq(&name) {
            continue;
        } else {
            return Ok(true);
        }
    }
    Ok(false)
}

/// Retrieves the current version being used.
///
/// This function reads the "used" file from the downloads directory, which
/// contains the current version being used. If the "used" file cannot be found,
/// it means that is not installed through hyper-jump.
///
/// # Returns
///
/// * `Result<String>` - Returns a `Result` that contains the current version
///   being used, or an error if the operation failed.
///
/// # Errors
///
/// This function will return an error if:
///
/// * The downloads directory cannot be retrieved.
/// * The "used" file cannot be read.
///
/// # Example
///
/// ```rust
/// let current_version = get_current_version().await.unwrap();
/// println!("The current version is {}", current_version);
/// ```
pub async fn get_current_version(package: Package) -> Result<String> {
    let mut downloads_dir = crate::fs::get_downloads_directory(package).await?;
    downloads_dir.push("used");
    println!("downloads_dir: {:?}", downloads_dir);
    tokio::fs::read_to_string(&downloads_dir)
        .await
        .map_err(|_| anyhow!("Could not read the current version"))
}

pub async fn is_version_used(version: &str, package: Package) -> bool {
    let current_version = get_current_version(package).await;
    match current_version {
        Ok(current_version) => current_version.eq(version),
        Err(_) => false,
    }
}

/// Switches to a specified version.
///
/// # Arguments
///
/// * `version` - The version to switch to.
/// * `package` - The package to switch versions for.
///
/// # Returns
///
/// * `Result<()>` - Returns a `Result` that indicates whether the operation was
///   successful or not.
///
/// # Errors
///
/// This function will return an error if:
///
/// * The downloads directory cannot be determined.
/// * The current directory cannot be changed to the downloads directory.
/// * The version cannot be written to the "used" file.
pub async fn switch_version(
    client: &reqwest::Client,
    version: &ParsedVersion,
    package: Package,
) -> Result<()> {
    std::env::set_current_dir(crate::fs::get_downloads_directory(package).await?)?;

    let file_version: String = version.tag_name.to_string();

    fs::write("used", &file_version).await?;

    Ok(())
}
