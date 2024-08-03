use reqwest::Client;

use crate::fs::get_file_type;
use crate::fs::get_platform_name;
use crate::fs::get_platform_name_download;
use crate::helpers::version::ParsedVersion;
use crate::helpers::version::VersionType;

const GITHUB_BASE_URL: &str = "https://github.com";
const GITHUB_API_BASE_URL: &str = "https://api.github.com/repos";
const CARDANO_DB_SYNC: &str = "IntersectMBO/cardano-db-sync";
const CARDANO_NODE_REPO: &str = "IntersectMBO/cardano-node";
const CARDANO_CLI_REPO: &str = "IntersectMBO/cardano-node";
const MITHRIL_REPO: &str = "input-output-hk/mithril";
const AIKEN_REPO: &str = "aiken-lang/aiken";
const OURA_REPO: &str = "txpipe/oura";
const DOLOS_REPO: &str = "txpipe/dolos";
const SCROLLS_REPO: &str = "txpipe/scrolls";

/// Represents the specification of a package.
///
/// * `alias` - Alias of the package.
/// * `version` - Optional version of the package.
/// * `binary_path` - Path to the binary of the package.
/// * `package_type` - Type of the package.
#[derive(Debug, Clone)]
pub struct Spec {
    pub alias: String,
    pub version: Option<ParsedVersion>,
    pub binary_path: String,
    pub package_type: PackageType,
}

/// Enum representing different types of packages.
///
/// * `Aiken` - Represents an Aiken package.
/// * `Mithril` - Represents a Mithril package.
/// * `CardanoCli` - Represents a Cardano CLI package.
/// * `CardanoNode` - Represents a Cardano Node package.
#[derive(Debug, Clone)]
pub enum Package {
    Oura(Spec),
    Aiken(Spec),
    Dolos(Spec),
    Mithril(Spec),
    Scrolls(Spec),
    CardanoCli(Spec),
    CardanoNode(Spec),
    CardanoDbSync(Spec),
    CardanoSubmitApi(Spec),
}

/// Enum representing different types of package types.
///
/// * `CardanoNode` - Represents the Cardano Node package type.
/// * `CardanoCli` - Represents the Cardano CLI package type.
/// * `Mithril` - Represents the Mithril package type.
/// * `Aiken` - Represents the Aiken package type.
#[derive(Debug, Clone)]
pub enum PackageType {
    Oura,
    Aiken,
    Dolos,
    Mithril,
    Scrolls,
    CardanoCli,
    CardanoNode,
    CardanoDbSync,
    CardanoSubmitApi,
}

/// Macro to create a `Package` variant with the appropriate `Spec` struct.
///
/// This macro simplifies the creation of different `Package` variants by
/// matching on the `PackageType` and constructing the corresponding `Spec`
/// struct with the provided alias and binary path.
///
/// # Parameters
/// - `$package_type`: The type of the package (of type `PackageType`).
/// - `$version`: The version of the package (of type `VersionType`).
/// - `$(($variant:ident, $alias:expr, $binary_path:expr)),*`: A list of tuples
///   where each tuple contains:
///   - `$variant`: The variant of the `PackageType` enum.
///   - `$alias`: The alias string for the package.
///   - `$binary_path`: The binary path string for the package.
macro_rules! create_package {
    ($package_type:expr, $version:expr, $(($variant:ident, $alias:expr, $binary_path:expr)),*) => {
        match $package_type {
            $(
                PackageType::$variant => Package::$variant(Spec {
                    alias: $alias,
                    version: $version,
                    binary_path: $binary_path,
                    package_type: $package_type,
                }),
            )*
        }
    };
}

impl PackageType {
    /// Creates a `PackageType` from a string.
    ///
    /// # Arguments
    ///
    /// * `package` - A string slice representing the package.
    ///
    /// Returns a `PackageType`.
    ///
    /// # Panics
    ///
    /// Panics if the provided string does not match any known package type.
    ///
    /// # Examples
    ///
    /// ```
    /// let package_type = PackageType::from_str("cardano-node"); 
    /// ```
    pub fn from_str(package: &str) -> Self {
        match package {
            "oura" => PackageType::Oura,
            "aiken" => PackageType::Aiken,
            "dolos" => PackageType::Dolos,
            "scrolls" => PackageType::Scrolls,
            "cardano-cli" => PackageType::CardanoCli,
            "cardano-node" => PackageType::CardanoNode,
            "mithril-client" => PackageType::Mithril,
            "cardano-db-sync" => PackageType::CardanoDbSync,
            "cardano-submit-api" => PackageType::CardanoSubmitApi,
            _ => panic!("Unknown package"),
        }
    }

    pub fn alias(&self) -> String {
        match self {
            PackageType::Oura => "oura".to_string(),
            PackageType::Aiken => "aiken".to_string(),
            PackageType::Dolos => "dolos".to_string(),
            PackageType::Scrolls => "scrolls".to_string(),
            PackageType::Mithril => "mithril-client".to_string(),
            PackageType::CardanoCli => "cardano-cli".to_string(),
            PackageType::CardanoNode => "cardano-node".to_string(),
            PackageType::CardanoDbSync => "cardano-db-sync".to_string(),
            PackageType::CardanoSubmitApi => "cardano-submit-api".to_string(),
        }
    }

    pub fn format_binary_path(&self, version: &str) -> String {
        let platform = get_platform_name_download(self.clone());
        match self {
            PackageType::CardanoSubmitApi => "bin".to_string(),
            PackageType::CardanoDbSync => "{version}".replace("{version}", version),
            PackageType::CardanoNode => "bin".to_string(),
            PackageType::CardanoCli => "bin".to_string(),
            PackageType::Mithril => "".to_string(),
            PackageType::Oura => "".to_string(),
            PackageType::Scrolls => "".to_string(),
            PackageType::Aiken => "aiken-{platform}".replace("{platform}", platform),
            PackageType::Dolos => "dolos-{platform}".replace("{platform}", platform),
        }
    }

    /// Returns the repository URL for the package type.
    ///
    /// # Returns
    ///
    /// A string slice representing the repository URL.
    ///
    /// # Examples
    ///
    /// ```
    /// let repo_url = PackageType::CardanoNode.repo(); 
    /// ```
    pub fn repo(&self) -> &str {
        match self {
            PackageType::Oura => OURA_REPO,
            PackageType::Aiken => AIKEN_REPO,
            PackageType::Dolos => DOLOS_REPO,
            PackageType::Scrolls => SCROLLS_REPO,
            PackageType::Mithril => MITHRIL_REPO,
            PackageType::CardanoCli => CARDANO_CLI_REPO,
            PackageType::CardanoNode => CARDANO_NODE_REPO,
            PackageType::CardanoDbSync => CARDANO_DB_SYNC,
            PackageType::CardanoSubmitApi => CARDANO_NODE_REPO,
        }
    }

    /// Returns the base URL for GitHub.
    ///
    /// # Returns
    ///
    /// A string slice representing the base URL.
    ///
    /// # Examples
    ///
    /// ```
    /// let base_url = PackageType::CardanoNode.base_url(); 
    /// ```
    pub fn base_url(&self) -> &str { GITHUB_BASE_URL }

    /// Returns the base URL for the GitHub API.
    ///
    /// # Returns
    ///
    /// A string slice representing the API base URL.
    ///
    /// # Examples
    ///
    /// ```
    /// let api_base_url = PackageType::CardanoNode.api_base_url(); 
    /// ```
    pub fn api_base_url(&self) -> &str { GITHUB_API_BASE_URL }

    /// Constructs the URL to get the latest release for the package type.
    ///
    /// # Returns
    ///
    /// A string representing the URL to get the latest release.
    ///
    /// # Examples
    ///
    /// ```
    /// let latest_url = PackageType::CardanoNode.get_latest_url(); 
    /// ```
    pub fn get_latest_url(&self) -> String {
        format!("{}/{}/releases/latest", self.api_base_url(), self.repo())
    }
}

/// Constructs a new `Package` with the specified type and version.
///
/// # Arguments
///
/// * `package_type` - The type of the package to construct.
///
/// * `version` - The version string of the package.
/// * `client` - An optional reference to a `reqwest::Client` for making HTTP
///   requests.
///
/// # Returns
///
/// Returns a new instance of `Package`.
impl Package {
    /// Returns the alias of the package.
    ///
    /// # Returns
    ///
    /// A string representing the alias of the package.
    ///
    /// # Examples
    ///
    /// ```
    /// let package = Package::CardanoNode(Spec {
    ///     alias: "cardano-node".to_string(),
    ///     ..Default::default()
    /// });
    /// let alias = package.alias();
    /// ```
    pub fn alias(&self) -> String {
        match self {
            Package::Oura(Spec { alias, .. }) => alias.clone(),
            Package::Aiken(Spec { alias, .. }) => alias.clone(),
            Package::Dolos(Spec { alias, .. }) => alias.clone(),
            Package::Mithril(Spec { alias, .. }) => alias.clone(),
            Package::Scrolls(Spec { alias, .. }) => alias.clone(),
            Package::CardanoCli(Spec { alias, .. }) => alias.clone(),
            Package::CardanoNode(Spec { alias, .. }) => alias.clone(),
            Package::CardanoDbSync(Spec { alias, .. }) => alias.clone(),
            Package::CardanoSubmitApi(Spec { alias, .. }) => alias.clone(),
        }
    }

    /// Returns the version of the package.
    ///
    /// # Returns
    ///
    /// An optional `ParsedVersion` representing the version of the package.
    ///
    /// # Examples
    ///
    /// ```
    /// let package = Package::CardanoNode(Spec {
    ///     version: Some(ParsedVersion::new("1.0.0")),
    ///     ..Default::default()
    /// });
    /// let version = package.version();
    /// ```
    pub fn version(&self) -> Option<ParsedVersion> {
        match self {
            Package::Oura(Spec { version, .. }) => version.clone(),
            Package::Aiken(Spec { version, .. }) => version.clone(),
            Package::Dolos(Spec { version, .. }) => version.clone(),
            Package::Scrolls(Spec { version, .. }) => version.clone(),
            Package::Mithril(Spec { version, .. }) => version.clone(),
            Package::CardanoCli(Spec { version, .. }) => version.clone(),
            Package::CardanoNode(Spec { version, .. }) => version.clone(),
            Package::CardanoDbSync(Spec { version, .. }) => version.clone(),
            Package::CardanoSubmitApi(Spec { version, .. }) => version.clone(),
        }
    }

    /// Returns the binary path of the package.
    ///
    /// # Returns
    ///
    /// A string representing the binary path of the package.
    ///
    /// # Examples
    ///
    /// ```
    /// let package = Package::CardanoNode(Spec {
    ///     binary_path: "bin".to_string(),
    ///     ..Default::default()
    /// });
    /// let binary_path = package.binary_path();
    /// ```
    pub fn binary_path(&self) -> String {
        match self {
            Package::Oura(Spec { binary_path, .. }) => binary_path.clone(),
            Package::Aiken(Spec { binary_path, .. }) => binary_path.clone(),
            Package::Dolos(Spec { binary_path, .. }) => binary_path.clone(),
            Package::Scrolls(Spec { binary_path, .. }) => binary_path.clone(),
            Package::Mithril(Spec { binary_path, .. }) => binary_path.clone(),
            Package::CardanoCli(Spec { binary_path, .. }) => binary_path.clone(),
            Package::CardanoNode(Spec { binary_path, .. }) => binary_path.clone(),
            Package::CardanoDbSync(Spec { binary_path, .. }) => binary_path.clone(),
            Package::CardanoSubmitApi(Spec { binary_path, .. }) => binary_path.clone(),
        }
    }
    // Returns the binary name of the package.
    ///
    /// # Returns
    ///
    /// A string representing the binary name of the package.
    ///
    /// # Examples
    ///
    /// ```
    /// let package = Package::CardanoNode(Spec {
    ///     alias: "cardano-node".to_string(),
    ///     ..Default::default()
    /// });
    /// let binary_name = package.binary_name();
    /// ```
    pub fn binary_name(&self) -> String {
        match self {
            Package::Oura(Spec { alias, .. }) => alias.clone(),
            Package::Aiken(Spec { alias, .. }) => alias.clone(),
            Package::Dolos(Spec { alias, .. }) => alias.clone(),
            Package::Scrolls(Spec { alias, .. }) => alias.clone(),
            Package::Mithril(Spec { alias, .. }) => alias.clone(),
            Package::CardanoCli(Spec { alias, .. }) => alias.clone(),
            Package::CardanoNode(Spec { alias, .. }) => alias.clone(),
            Package::CardanoDbSync(Spec { alias, .. }) => alias.clone(),
            Package::CardanoSubmitApi(Spec { alias, .. }) => alias.clone(),
        }
    }

    /// Returns the package type of the package.
    ///
    /// # Returns
    ///
    /// A `PackageType` representing the type of the package.
    ///
    /// # Examples
    ///
    /// ```
    /// let package = Package::CardanoNode(Spec {
    ///     package_type: PackageType::CardanoNode,
    ///     ..Default::default()
    /// });
    /// let package_type = package.package_type();
    /// ```
    pub fn package_type(&self) -> PackageType {
        match self {
            Package::Oura(Spec { package_type, .. }) => package_type.clone(),
            Package::Aiken(Spec { package_type, .. }) => package_type.clone(),
            Package::Dolos(Spec { package_type, .. }) => package_type.clone(),
            Package::Scrolls(Spec { package_type, .. }) => package_type.clone(),
            Package::Mithril(Spec { package_type, .. }) => package_type.clone(),
            Package::CardanoCli(Spec { package_type, .. }) => package_type.clone(),
            Package::CardanoNode(Spec { package_type, .. }) => package_type.clone(),
            Package::CardanoDbSync(Spec { package_type, .. }) => package_type.clone(),
            Package::CardanoSubmitApi(Spec { package_type, .. }) => package_type.clone(),
        }
    }

    /// Constructs the template URL for the package.
    ///
    /// # Returns
    ///
    /// A string representing the template URL for the package.
    ///
    /// # Examples
    ///
    /// ```
    /// let package = Package::CardanoNode(Spec {
    ///     package_type: PackageType::CardanoNode,
    ///     ..Default::default()
    /// });
    /// let template_url = package.get_template_url();
    /// ```
    pub fn get_template_url(&self) -> String {
        let p = self.package_type();
        let base = p.base_url();
        let repo = p.repo();
        match p {
            PackageType::CardanoSubmitApi => format!(
                "{}/{}/releases/download/{{version}}/cardano-node-{{version}}-{{OS}}.{{file_type}}",
                base, repo,
            ),
            PackageType::CardanoNode => format!(
                "{}/{}/releases/download/{{version}}/cardano-node-{{version}}-{{OS}}.{{file_type}}",
                base, repo,
            ),
            PackageType::CardanoDbSync => format!(
                "{}/{}/releases/download/{{version}}/cardano-db-sync-{{version}}-{{platform}}.\
                 {{file_type}}",
                base, repo,
            ),
            PackageType::CardanoCli => format!(
                "{}/{}/releases/download/{{version}}/cardano-node-{{version}}-{{OS}}.{{file_type}}",
                base, repo,
            ),
            PackageType::Mithril => format!(
                "{}/{}/releases/download/{{version}}/mithril-{{version}}-{{OS}}-{{platform}}.\
                 {{file_type}}",
                base, repo,
            ),
            PackageType::Scrolls => format!(
                "{}/{}/releases/download/{{version}}/scrolls-{{platform}}.{{file_type}}",
                base, repo,
            ),
            PackageType::Aiken => format!(
                "{}/{}/releases/download/{{version}}/aiken-{{platform}}.{{file_type}}",
                base, repo,
            ),
            PackageType::Dolos => format!(
                "{}/{}/releases/download/{{version}}/dolos-{{platform}}.{{file_type}}",
                base, repo,
            ),
            PackageType::Oura => format!(
                "{}/{}/releases/download/{{version}}/oura-{{platform}}.{{file_type}}",
                base, repo,
            ),
        }
    }

    /// Constructs the download URL for the package.
    ///
    /// # Returns
    ///
    /// A string representing the download URL for the package.
    ///
    /// # Panics
    ///
    /// Panics if the version is not set.
    ///
    /// # Examples
    ///
    /// ```
    /// let package = Package::CardanoNode(Spec {
    ///     version: Some(ParsedVersion::new("1.0.0")),
    ///     ..Default::default()
    /// });
    /// let download_url = package.download_url();
    /// ```
    pub fn download_url(&self) -> String {
        let v = self.version().expect("Version not set");
        let p = self.package_type();

        self.get_template_url()
            .replace("{version}", v.non_parsed_string.as_str())
            .replace("{OS}", get_platform_name())
            .replace("{platform}", get_platform_name_download(p))
            .replace("{file_type}", get_file_type(self.package_type()))
    }

    /// Constructs the releases URL for the package.
    ///
    /// # Returns
    ///
    /// A string representing the releases URL for the package.
    ///
    /// # Examples
    ///
    /// ```
    /// let package = Package::CardanoNode(Spec {
    ///     package_type: PackageType::CardanoNode,
    ///     ..Default::default()
    /// });
    /// let releases_url = package.releases_url();
    /// ```
    pub fn releases_url(&self) -> String {
        let p = self.package_type();
        format!("{}/{}/releases", p.api_base_url(), p.repo())
    }

    /// Creates a new instance of `Package`.
    ///
    /// # Arguments
    ///
    /// * `package_type` - The type of the package to construct.
    /// * `version` - The version string of the package.
    /// * `client` - An optional reference to a `reqwest::Client` for making
    ///   HTTP requests.
    ///
    /// # Returns
    ///
    /// Returns a new instance of `Package`.
    ///
    /// # Examples
    ///
    /// ```
    /// let client = Client::new();
    /// let package = Package::new(PackageType::CardanoNode, "1.0.0".to_string(), Some(&client)).await;
    /// ```
    pub async fn new(package_type: PackageType, version: String, client: Option<&Client>) -> Self {
        let version = VersionType::parse(&version, client, package_type.clone()).await.unwrap();
        let binary_path = package_type.format_binary_path(version.non_parsed_string.as_str());
        let alias = package_type.alias();
        create_package!(
            package_type,
            Some(version),
            (Oura, alias, binary_path),
            (Aiken, alias, binary_path),
            (Dolos, alias, binary_path),
            (Scrolls, alias, binary_path),
            (Mithril, alias, binary_path),
            (CardanoCli, alias, binary_path),
            (CardanoNode, alias, binary_path),
            (CardanoDbSync, alias, binary_path),
            (CardanoSubmitApi, alias, binary_path)
        )
    }
}
