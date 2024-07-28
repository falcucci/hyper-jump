pub mod aiken;
pub mod cardano_cli;
pub mod cardano_node;
pub mod mithril;

pub const CARDANO_NODE_PACKAGE_DOWNLOAD_URL: &str = "https://github.com/IntersectMBO/cardano-node/releases/download/{version}/cardano-node-{version}-{OS}.tar.gz";
pub const CARDANO_NODE_PACKAGE_RELEASES_URL: &str =
    "https://api.github.com/repos/IntersectMBO/cardano-node/releases";
pub const CARDANO_NODE_PACKAGE_LATEST_RELEASE_URL: &str =
    "https://api.github.com/repos/IntersectMBO/cardano-node/releases/latest";

pub const CARDANO_CLI_PACKAGE_DOWNLOAD_URL: &str = "https://github.com/IntersectMBO/cardano-node/releases/download/{version}/cardano-node-{version}-{OS}.{file_type}";
pub const CARDANO_CLI_PACKAGE_RELEASES_URL: &str =
    "https://api.github.com/repos/IntersectMBO/cardano-node/releases";
pub const CARDANO_CLI_PACKAGE_LATEST_RELEASE_URL: &str =
    "https://api.github.com/repos/IntersectMBO/cardano-node/releases/latest";

pub const MITHRIL_PACKAGE_DOWNLOAD_URL: &str = "https://github.com//input-output-hk/mithril/releases/download/{version}/mithril-{version}-{OS}-{platform}.{file_type}";
pub const MITHRIL_PACKAGE_RELEASES_URL: &str =
    "https://api.github.com/repos/input-output-hk/mithril/releases";
pub const MITHRIL_PACKAGE_LATEST_RELEASE_URL: &str =
    "https://api.github.com/repos/input-output-hk/mithril/releases/latest";

pub const AIKEN_PACKAGE_DOWNLOAD_URL: &str =
    "https://github.com/aiken-lang/aiken/releases/download/{version}/aiken-{platform}.{file_type}";
pub const AIKEN_PACKAGE_RELEASES_URL: &str =
    "https://api.github.com/repos/aiken-lang/aiken/releases";
pub const AIKEN_PACKAGE_LATEST_RELEASE_URL: &str =
    "https://api.github.com/repos/aiken-lang/aiken/releases/latest";
