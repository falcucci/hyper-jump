use tracing::info;

use crate::adapters::archive::LocalArchive;
use crate::adapters::downloader::ReqwestDownloader;
use crate::adapters::fs::TokioFs;
use crate::adapters::github_release::GitHubReleaseProvider;
use crate::app::install;
use crate::app::resolve::resolve_requested_version;
use crate::domain::package::PackageType;
use crate::ports::ProxyInstaller;
use crate::ports::UsedVersionStore;

#[derive(clap::Parser)]
pub struct Args {
    #[command(subcommand)]
    command: Commands,
}

#[derive(clap::Parser)]
pub enum Commands {
    Reth { version: String },
    Oura { version: String },
    Aiken { version: String },
    Dolos { version: String },
    Neovim { version: String },
    Zellij { version: String },
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
  ($command:expr, $client:expr, $paths:expr, $platform:expr, $(($variant:ident, $package_type:expr)),*) => {
    match $command {
      $(
        Commands::$variant { version } => {
          use_cmd($client, $package_type, version, $paths, $platform)
            .await
            .expect("Failed to use");
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
    let paths = crate::adapters::path::FsPaths::new(ctx.dirs.root_dir.clone());
    let platform = crate::adapters::platform::StdPlatform;
    execute!(
        args.command,
        client,
        &paths,
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

pub async fn use_cmd(
    client: Option<&reqwest::Client>,
    package_type: PackageType,
    requested_version: String,
    paths: &crate::adapters::path::FsPaths,
    platform: &impl crate::ports::Platform,
) -> Result<(), Box<dyn std::error::Error>> {
    let provider = GitHubReleaseProvider::new(client);
    let downloader = ReqwestDownloader::new(client);
    let archive = LocalArchive;
    let fs = TokioFs;
    let lock = crate::adapters::lock::FileLock::from_paths(paths).await?;
    let proxy = crate::adapters::proxy::ProxyFsCopier::new(
        paths.clone(),
        crate::adapters::env::StdEnv,
        crate::adapters::fs::TokioFs,
        crate::adapters::process::TokioProcess,
    );
    let used_store = crate::adapters::used_store::UsedFileStore::new(paths.clone());

    let parsed_version =
        resolve_requested_version(&requested_version, package_type.clone(), &provider).await?;
    let binary_path = crate::app::layout::binary_path(package_type.clone(), platform);
    let package = crate::domain::package::Package::with_parsed(
        package_type,
        parsed_version.clone(),
        binary_path,
    );
    let version = parsed_version;
    let is_version_used = match used_store.current(package.clone()).await? {
        Some(current) => current == version.tag_name,
        None => false,
    };

    proxy.ensure_proxy(&package.alias()).await?;

    if is_version_used {
        info!("{} is already in use.", version.tag_name);
        return Ok(());
    }

    install::install(
        package.package_type(),
        version.tag_name.clone(),
        &provider,
        &downloader,
        &archive,
        &fs,
        platform,
        &lock,
        &used_store,
        paths,
        &proxy,
    )
    .await?;

    used_store.set_current(package.clone(), &version.tag_name).await?;

    info!("You can now use {}!", version.tag_name);

    Ok(())
}
