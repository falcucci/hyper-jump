use crate::domain::version::ParsedVersion;

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
#[derive(Debug, Clone)]
pub struct Spec {
    pub alias: String,
    pub version: Option<ParsedVersion>,
    pub binary_path: String,
    pub package_type: PackageType,
}

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
    PartnerChainNode(Spec),
    CardanoSubmitApi(Spec),
}

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
    PartnerChainNode,
    CardanoSubmitApi,
}

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
            PackageType::PartnerChainNode => "partner-chains-node".to_string(),
            PackageType::CardanoSubmitApi => "cardano-submit-api".to_string(),
        }
    }

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
            PackageType::PartnerChainNode => PARTNER_CHAIN_CLI_REPO,
        }
    }

    pub fn base_url(&self) -> &str { GITHUB_BASE_URL }

    pub fn api_base_url(&self) -> &str { GITHUB_API_BASE_URL }

    pub fn get_latest_url(&self) -> String {
        format!("{}/{}/releases/latest", self.api_base_url(), self.repo())
    }
}

impl Package {
    pub fn from_type(package_type: PackageType, binary_path: String) -> Self {
        let alias = package_type.alias();
        create_package!(
            package_type,
            None,
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
            (PartnerChainNode, alias, binary_path),
            (CardanoSubmitApi, alias, binary_path)
        )
    }

    pub fn with_parsed(
        package_type: PackageType,
        version: ParsedVersion,
        binary_path: String,
    ) -> Self {
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
            (PartnerChainNode, alias, binary_path),
            (CardanoSubmitApi, alias, binary_path)
        )
    }

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
            Package::PartnerChainNode(Spec { alias, .. }) => alias.clone(),
            Package::CardanoSubmitApi(Spec { alias, .. }) => alias.clone(),
        }
    }

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
            Package::PartnerChainNode(Spec { version, .. }) => version.clone(),
            Package::CardanoSubmitApi(Spec { version, .. }) => version.clone(),
        }
    }

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
            Package::PartnerChainNode(Spec { binary_path, .. }) => binary_path.clone(),
            Package::CardanoSubmitApi(Spec { binary_path, .. }) => binary_path.clone(),
        }
    }

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
            Package::PartnerChainNode(Spec { alias, .. }) => alias.clone(),
            Package::CardanoSubmitApi(Spec { alias, .. }) => alias.clone(),
        }
    }

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
            Package::PartnerChainNode(Spec { package_type, .. }) => package_type.clone(),
            Package::CardanoSubmitApi(Spec { package_type, .. }) => package_type.clone(),
        }
    }

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

    // download_url moved to app layer to keep domain independent of platform/ports.
}
