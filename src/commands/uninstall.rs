use clap::Parser;

use crate::adapters::fs::TokioFs;
use crate::adapters::github_release::GitHubReleaseProvider;
use crate::adapters::path::FsPaths;
use crate::adapters::used_store::UsedFileStore;
use crate::app::uninstall::uninstall_requested;
use crate::domain::package::PackageType;

#[derive(Parser)]
pub struct Args {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Parser)]
pub enum Commands {
    Reth { version: String },
    Oura { version: String },
    Aiken { version: String },
    Dolos { version: String },
    Zellij { version: String },
    Neovim { version: String },
    Jujutsu { version: String },
    Mithril { version: String },
    Scrolls { version: String },
    CardanoCli { version: String },
    CardanoNode { version: String },
    SidechainCli { version: String },
    PartnerChainNode { version: String },
    CardanoSubmitApi { version: String },
}

/// A macro to execute an uninstall command based on the provided variant and
/// package type.
///
/// This macro matches the provided command against a list of command variants
/// and executes the corresponding code for each variant. It creates a new
/// `Package` instance with the specified package type, version, and client, and
/// then uninstalls the package.
///
/// # Parameters
///
/// - `$command`: The command to be matched and executed. The command must
///   include a `version`.
/// - `$client`: The client to be used for creating the `Package`.
/// - `$(($variant:ident, $package_type:expr)),*`: A list of tuples containing
///   the command variant and the corresponding package type.
macro_rules! execute {
    ($command:expr, $client:expr, $paths:expr, $used:expr, $fs:expr, $platform:expr, $(($variant:ident, $package_type:expr)),*) => {
        match $command {
            $(
                Commands::$variant { version } => {
                    let provider = GitHubReleaseProvider::new($client);
                    uninstall_requested(
                        $package_type,
                        version,
                        &provider,
                        $platform,
                        &$fs,
                        &$paths,
                        &$used,
                    )
                    .await
                    .expect("Failed to uninstall")
                }
            )*
        }
    }
}

pub async fn run(
    args: Args,
    ctx: &crate::Context,
    client: Option<&reqwest::Client>,
) -> miette::Result<()> {
    let paths = FsPaths::new(ctx.dirs.root_dir.clone());
    let used_store = UsedFileStore::new(paths.clone());
    let fs = TokioFs;
    let platform = crate::adapters::platform::StdPlatform;
    execute!(
        args.command,
        client,
        paths,
        used_store,
        fs,
        &platform,
        (Reth, PackageType::Reth),
        (Oura, PackageType::Oura),
        (Aiken, PackageType::Aiken),
        (Dolos, PackageType::Dolos),
        (Zellij, PackageType::Zellij),
        (Neovim, PackageType::Neovim),
        (Jujutsu, PackageType::Jujutsu),
        (Mithril, PackageType::Mithril),
        (Scrolls, PackageType::Scrolls),
        (CardanoCli, PackageType::CardanoCli),
        (CardanoNode, PackageType::CardanoNode),
        (SidechainCli, PackageType::SidechainCli),
        (PartnerChainNode, PackageType::PartnerChainNode),
        (CardanoSubmitApi, PackageType::CardanoSubmitApi)
    );

    Ok(())
}
