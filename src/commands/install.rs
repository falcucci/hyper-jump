use clap::Parser;

use crate::adapters::archive::LocalArchive;
use crate::adapters::downloader::ReqwestDownloader;
use crate::adapters::fs::TokioFs;
use crate::adapters::github_release::GitHubReleaseProvider;
use crate::app::install as app_install;
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
    NeoVim { version: String },
    Jujutsu { version: String },
    Mithril { version: String },
    Scrolls { version: String },
    CardanoCli { version: String },
    CardanoNode { version: String },
    SidechainCli { version: String },
    PartnerChainNode { version: String },
    CardanoSubmitApi { version: String },
}

macro_rules! execute {
    ($command:expr, $provider:expr, $downloader:expr, $archive:expr, $fs:expr, $platform:expr, $lock:expr, $used:expr, $paths:expr, $proxy:expr) => {{
        let (package_type, version) = match $command {
            Commands::Reth { version } => (PackageType::Reth, version),
            Commands::Oura { version } => (PackageType::Oura, version),
            Commands::Aiken { version } => (PackageType::Aiken, version),
            Commands::Dolos { version } => (PackageType::Dolos, version),
            Commands::Zellij { version } => (PackageType::Zellij, version),
            Commands::NeoVim { version } => (PackageType::Neovim, version),
            Commands::Jujutsu { version } => (PackageType::Jujutsu, version),
            Commands::Mithril { version } => (PackageType::Mithril, version),
            Commands::Scrolls { version } => (PackageType::Scrolls, version),
            Commands::CardanoCli { version } => (PackageType::CardanoCli, version),
            Commands::CardanoNode { version } => (PackageType::CardanoNode, version),
            Commands::SidechainCli { version } => (PackageType::SidechainCli, version),
            Commands::PartnerChainNode { version } => (PackageType::PartnerChainNode, version),
            Commands::CardanoSubmitApi { version } => (PackageType::CardanoSubmitApi, version),
        };

        app_install::install(
            package_type,
            version,
            $provider,
            $downloader,
            $archive,
            $fs,
            $platform,
            $lock,
            $used,
            $paths,
            $proxy,
        )
        .await
    }};
}

pub async fn run(
    args: Args,
    ctx: &crate::Context,
    client: Option<&reqwest::Client>,
) -> miette::Result<()> {
    let provider = GitHubReleaseProvider::new(client);
    let downloader = ReqwestDownloader::new(client);
    let archive = LocalArchive;
    let fs = TokioFs;
    let platform = crate::adapters::platform::StdPlatform;
    let paths = crate::adapters::path::FsPaths::new(ctx.dirs.root_dir.clone());
    let lock = crate::adapters::lock::FileLock::from_paths(&paths)
        .await
        .map_err(|e| miette::miette!(e))?;
    let proxy = crate::adapters::proxy::ProxyFsCopier::new(
        paths.clone(),
        crate::adapters::env::StdEnv,
        crate::adapters::fs::TokioFs,
        crate::adapters::process::TokioProcess,
    );
    let used_store = crate::adapters::used_store::UsedFileStore::new(paths.clone());

    execute!(
        args.command,
        &provider,
        &downloader,
        &archive,
        &fs,
        &platform,
        &lock,
        &used_store,
        &paths,
        &proxy
    )
    .map_err(|e| miette::miette!(e))
}
