pub mod cardano_cli;
pub mod cardano_node;
pub mod mithril;

pub const CARDANO_NODE_PACKAGE_URL: &str =
    "https://github.com/IntersectMBO/cardano-node/releases/download/{version}/cardano-node-{version}-macos.tar.gz";

pub const CARDANO_CLI_PACKAGE_URL: &str =
    "https://github.com/IntersectMBO/cardano-cli/releases/download/cardano-cli-{version}/cardano-cli-{version}-aarch64-darwin.tar.gz";
