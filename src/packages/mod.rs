use reqwest::Client;

use crate::fs::get_file_type;
use crate::fs::get_platform_name;
use crate::fs::get_platform_name_download;
use crate::helpers::version::ParsedVersion;
use crate::helpers::version::VersionType;

const GITHUB_BASE_URL: &str = "https://github.com";
const GITHUB_API_BASE_URL: &str = "https://api.github.com/repos";
const PARTNER_CHAIN_CLI_REPO: &str = "input-output-hk/partner-chains";
const CARDANO_NODE_REPO: &str = "IntersectMBO/cardano-node";
const CARDANO_CLI_REPO: &str = "IntersectMBO/cardano-node";
const JUJUTSU_REPO: &str = "jj-vcs/jj";
const MITHRIL_REPO: &str = "input-output-hk/mithril";
const ZELLIJ_REPO: &str = "zellij-org/zellij";
const NEOVIM_REPO: &str = "neovim/neovim";
const AIKEN_REPO: &str = "aiken-lang/aiken";
const OURA_REPO: &str = "txpipe/oura";
const DOLOS_REPO: &str = "txpipe/dolos";
const RETH_REPO: &str = "paradigmxyz/reth";
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
    Reth(Spec),
    Oura(Spec),
    Aiken(Spec),
    Dolos(Spec),
    Zellij(Spec),
    Neovim(Spec),
    Jujutsu(Spec),
    Mithril(Spec),
    Scrolls(Spec),
    CardanoCli(Spec),
    CardanoNode(Spec),
    SidechainCli(Spec),
    PartnerChainCli(Spec),
    PartnerChainNode(Spec),
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
    Reth,
    Oura,
    Aiken,
    Dolos,
    Zellij,
    Neovim,
    Jujutsu,
    Mithril,
    Scrolls,
    CardanoCli,
    CardanoNode,
    SidechainCli,
    PartnerChainCli,
    PartnerChainNode,
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
            "reth" => PackageType::Reth,
            "oura" => PackageType::Oura,
            "aiken" => PackageType::Aiken,
            "dolos" => PackageType::Dolos,
            "zellij" => PackageType::Zellij,
            "nvim" => PackageType::Neovim,
            "scrolls" => PackageType::Scrolls,
            "cardano-cli" => PackageType::CardanoCli,
            "cardano-node" => PackageType::CardanoNode,
            "jj" => PackageType::Jujutsu,
            "mithril-client" => PackageType::Mithril,
            "sidechain-main-cli" => PackageType::SidechainCli,
            "partner-chains-cli" => PackageType::PartnerChainCli,
            "partner-chains-node" => PackageType::PartnerChainNode,
            "cardano-submit-api" => PackageType::CardanoSubmitApi,
            _ => panic!("Unknown package"),
        }
    }

    pub fn alias(&self) -> String {
        match self {
            PackageType::Reth => "reth".to_string(),
            PackageType::Oura => "oura".to_string(),
            PackageType::Aiken => "aiken".to_string(),
            PackageType::Dolos => "dolos".to_string(),
            PackageType::Zellij => "zellij".to_string(),
            PackageType::Neovim => "nvim".to_string(),
            PackageType::Scrolls => "scrolls".to_string(),
            PackageType::Jujutsu => "jj".to_string(),
            PackageType::Mithril => "mithril-client".to_string(),
            PackageType::CardanoCli => "cardano-cli".to_string(),
            PackageType::CardanoNode => "cardano-node".to_string(),
            PackageType::SidechainCli => "sidechain-main-cli".to_string(),
            PackageType::PartnerChainCli => "partner-chains-cli".to_string(),
            PackageType::PartnerChainNode => "partner-chains-node".to_string(),
            PackageType::CardanoSubmitApi => "cardano-submit-api".to_string(),
        }
    }

    pub fn format_binary_path(&self) -> String {
        let platform = get_platform_name_download(self.clone());
        let os = get_platform_name();
        match self {
            PackageType::CardanoSubmitApi => "bin".to_string(),
            PackageType::PartnerChainNode => "".to_string(),
            PackageType::PartnerChainCli => "".to_string(),
            PackageType::SidechainCli => "".to_string(),
            PackageType::CardanoNode => "bin".to_string(),
            PackageType::CardanoCli => "bin".to_string(),
            PackageType::Jujutsu => "".to_string(),
            PackageType::Mithril => "".to_string(),
            PackageType::Zellij => "".to_string(),
            PackageType::Neovim => {
                format!("nvim-{os}-{platform}/bin", os = os, platform = platform)
            }
            PackageType::Oura => "".to_string(),
            PackageType::Scrolls => "".to_string(),
            PackageType::Aiken => format!("aiken-{platform}", platform = platform),
            PackageType::Dolos => format!("dolos-{platform}", platform = platform),
            PackageType::Reth => "".to_string(),
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
            PackageType::Reth => RETH_REPO,
            PackageType::Oura => OURA_REPO,
            PackageType::Aiken => AIKEN_REPO,
            PackageType::Dolos => DOLOS_REPO,
            PackageType::Zellij => ZELLIJ_REPO,
            PackageType::Neovim => NEOVIM_REPO,
            PackageType::Scrolls => SCROLLS_REPO,
            PackageType::Jujutsu => JUJUTSU_REPO,
            PackageType::Mithril => MITHRIL_REPO,
            PackageType::CardanoCli => CARDANO_CLI_REPO,
            PackageType::CardanoNode => CARDANO_NODE_REPO,
            PackageType::CardanoSubmitApi => CARDANO_NODE_REPO,
            PackageType::SidechainCli => PARTNER_CHAIN_CLI_REPO,
            PackageType::PartnerChainCli => PARTNER_CHAIN_CLI_REPO,
            PackageType::PartnerChainNode => PARTNER_CHAIN_CLI_REPO,
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
            Package::Reth(Spec { alias, .. }) => alias.clone(),
            Package::Oura(Spec { alias, .. }) => alias.clone(),
            Package::Aiken(Spec { alias, .. }) => alias.clone(),
            Package::Dolos(Spec { alias, .. }) => alias.clone(),
            Package::Zellij(Spec { alias, .. }) => alias.clone(),
            Package::Neovim(Spec { alias, .. }) => alias.clone(),
            Package::Jujutsu(Spec { alias, .. }) => alias.clone(),
            Package::Mithril(Spec { alias, .. }) => alias.clone(),
            Package::Scrolls(Spec { alias, .. }) => alias.clone(),
            Package::CardanoCli(Spec { alias, .. }) => alias.clone(),
            Package::CardanoNode(Spec { alias, .. }) => alias.clone(),
            Package::SidechainCli(Spec { alias, .. }) => alias.clone(),
            Package::PartnerChainCli(Spec { alias, .. }) => alias.clone(),
            Package::PartnerChainNode(Spec { alias, .. }) => alias.clone(),
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
            Package::Reth(Spec { version, .. }) => version.clone(),
            Package::Oura(Spec { version, .. }) => version.clone(),
            Package::Aiken(Spec { version, .. }) => version.clone(),
            Package::Dolos(Spec { version, .. }) => version.clone(),
            Package::Zellij(Spec { version, .. }) => version.clone(),
            Package::Neovim(Spec { version, .. }) => version.clone(),
            Package::Scrolls(Spec { version, .. }) => version.clone(),
            Package::Mithril(Spec { version, .. }) => version.clone(),
            Package::Jujutsu(Spec { version, .. }) => version.clone(),
            Package::CardanoCli(Spec { version, .. }) => version.clone(),
            Package::CardanoNode(Spec { version, .. }) => version.clone(),
            Package::SidechainCli(Spec { version, .. }) => version.clone(),
            Package::PartnerChainCli(Spec { version, .. }) => version.clone(),
            Package::PartnerChainNode(Spec { version, .. }) => version.clone(),
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
            Package::Reth(Spec { binary_path, .. }) => binary_path.clone(),
            Package::Oura(Spec { binary_path, .. }) => binary_path.clone(),
            Package::Aiken(Spec { binary_path, .. }) => binary_path.clone(),
            Package::Dolos(Spec { binary_path, .. }) => binary_path.clone(),
            Package::Zellij(Spec { binary_path, .. }) => binary_path.clone(),
            Package::Neovim(Spec { binary_path, .. }) => binary_path.clone(),
            Package::Scrolls(Spec { binary_path, .. }) => binary_path.clone(),
            Package::Jujutsu(Spec { binary_path, .. }) => binary_path.clone(),
            Package::Mithril(Spec { binary_path, .. }) => binary_path.clone(),
            Package::CardanoCli(Spec { binary_path, .. }) => binary_path.clone(),
            Package::CardanoNode(Spec { binary_path, .. }) => binary_path.clone(),
            Package::SidechainCli(Spec { binary_path, .. }) => binary_path.clone(),
            Package::PartnerChainCli(Spec { binary_path, .. }) => binary_path.clone(),
            Package::PartnerChainNode(Spec { binary_path, .. }) => binary_path.clone(),
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
            Package::Reth(Spec { alias, .. }) => alias.clone(),
            Package::Oura(Spec { alias, .. }) => alias.clone(),
            Package::Aiken(Spec { alias, .. }) => alias.clone(),
            Package::Dolos(Spec { alias, .. }) => alias.clone(),
            Package::Zellij(Spec { alias, .. }) => alias.clone(),
            Package::Neovim(Spec { alias, .. }) => alias.clone(),
            Package::Scrolls(Spec { alias, .. }) => alias.clone(),
            Package::Jujutsu(Spec { alias, .. }) => alias.clone(),
            Package::Mithril(Spec { alias, .. }) => alias.clone(),
            Package::CardanoCli(Spec { alias, .. }) => alias.clone(),
            Package::CardanoNode(Spec { alias, .. }) => alias.clone(),
            Package::SidechainCli(Spec { alias, .. }) => alias.clone(),
            Package::PartnerChainCli(Spec { alias, .. }) => alias.clone(),
            Package::PartnerChainNode(Spec { alias, .. }) => alias.clone(),
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
            Package::Reth(Spec { package_type, .. }) => package_type.clone(),
            Package::Oura(Spec { package_type, .. }) => package_type.clone(),
            Package::Aiken(Spec { package_type, .. }) => package_type.clone(),
            Package::Dolos(Spec { package_type, .. }) => package_type.clone(),
            Package::Zellij(Spec { package_type, .. }) => package_type.clone(),
            Package::Neovim(Spec { package_type, .. }) => package_type.clone(),
            Package::Scrolls(Spec { package_type, .. }) => package_type.clone(),
            Package::Jujutsu(Spec { package_type, .. }) => package_type.clone(),
            Package::Mithril(Spec { package_type, .. }) => package_type.clone(),
            Package::CardanoCli(Spec { package_type, .. }) => package_type.clone(),
            Package::CardanoNode(Spec { package_type, .. }) => package_type.clone(),
            Package::SidechainCli(Spec { package_type, .. }) => package_type.clone(),
            Package::PartnerChainCli(Spec { package_type, .. }) => package_type.clone(),
            Package::PartnerChainNode(Spec { package_type, .. }) => package_type.clone(),
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
            PackageType::PartnerChainNode => format!(
                "{}/{}/releases/download/{{version}}/{{OS}}_{{platform}}.{{file_type}}",
                base, repo,
            ),
            PackageType::PartnerChainCli => format!(
                "{}/{}/releases/download/{{version}}/{{OS}}_{{platform}}.{{file_type}}",
                base, repo,
            ),
            PackageType::SidechainCli => format!(
                "{}/{}/releases/download/{{version}}/{{OS}}_{{platform}}.{{file_type}}",
                base, repo,
            ),
            PackageType::CardanoNode => format!(
                "{}/{}/releases/download/{{version}}/cardano-node-{{version}}-{{OS}}.{{file_type}}",
                base, repo,
            ),
            PackageType::CardanoCli => format!(
                "{}/{}/releases/download/{{version}}/cardano-node-{{version}}-{{OS}}.{{file_type}}",
                base, repo,
            ),
            PackageType::Jujutsu => format!(
                "{}/{}/releases/download/{{version}}/jj-{{version}}-{{platform}}.{{file_type}}",
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
            PackageType::Zellij => format!(
                "{}/{}/releases/download/{{version}}/zellij-{{platform}}.{{file_type}}",
                base, repo,
            ),
            PackageType::Neovim => format!(
                "{}/{}/releases/download/{{version}}/nvim-{{OS}}-{{platform}}.{{file_type}}",
                base, repo,
            ),
            PackageType::Oura => format!(
                "{}/{}/releases/download/{{version}}/oura-{{platform}}.{{file_type}}",
                base, repo,
            ),
            PackageType::Reth => format!(
                "{}/{}/releases/download/{{version}}/reth-{{version}}-{{platform}}.{{file_type}}",
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
        let binary_path = package_type.format_binary_path();
        let alias = package_type.alias();
        create_package!(
            package_type,
            Some(version),
            (Reth, alias, binary_path),
            (Oura, alias, binary_path),
            (Aiken, alias, binary_path),
            (Dolos, alias, binary_path),
            (Zellij, alias, binary_path),
            (Neovim, alias, binary_path),
            (Scrolls, alias, binary_path),
            (Jujutsu, alias, binary_path),
            (Mithril, alias, binary_path),
            (CardanoCli, alias, binary_path),
            (CardanoNode, alias, binary_path),
            (SidechainCli, alias, binary_path),
            (PartnerChainCli, alias, binary_path),
            (PartnerChainNode, alias, binary_path),
            (CardanoSubmitApi, alias, binary_path)
        )
    }
}
