use semver::Version;

/// Represents a parsed version of the software.
///
/// This struct contains information about a parsed version of the software, including the tag name, version type, non-parsed string, and semantic version.
///
/// # Fields
///
/// * `tag_name: String` - The tag name of the parsed version.
/// * `version_type: VersionType` - The type of the parsed version.
/// * `non_parsed_string: String` - The non-parsed string of the parsed version.
/// * `semver: Option<Version>` - The semantic version of the parsed version, or `None` if the version is not a semantic version.
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
#[derive(Debug)]
pub struct ParsedVersion {
  pub tag_name: String,
  pub version_type: VersionType,
  pub non_parsed_string: String,
  pub semver: Option<Version>,
}

/// Represents the type of a software version.
///
/// This enum is used to distinguish between different types of software versions, such as normal versions, the latest version, nightly versions, versions identified by a hash, and nightly versions that have been rolled back.
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
/// let version_type = VersionType::Nightly;
/// match version_type {
///     VersionType::Normal => println!("This is a normal version."),
///     VersionType::Latest => println!("This is the latest version."),
///     VersionType::Hash => println!("This is a version identified by a hash."),
/// }
/// ```
#[derive(PartialEq, Eq, Debug)]
pub enum VersionType {
  Normal,
  Latest,
  Hash,
}
