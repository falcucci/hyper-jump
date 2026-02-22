use tracing::info;

use crate::adapters::archive::LocalArchive;
use crate::adapters::downloader::ReqwestDownloader;
use crate::adapters::fs::TokioFs;
use crate::adapters::github_release::GitHubReleaseProvider;
use crate::app::install;
use crate::app::resolve::resolve_requested_version;
use crate::domain::package::Package;
use crate::domain::package::PackageSpec;
use crate::ports::ProxyInstaller;
use crate::ports::UsedVersionStore;

#[derive(clap::Parser)]
pub struct Args {
    pub package: String,
    pub version: String,
}

pub async fn run(
    args: Args,
    ctx: &crate::Context,
    client: Option<&reqwest::Client>,
) -> miette::Result<()> {
    let spec = ctx.packages.resolve(&args.package).map_err(|e| miette::miette!(e))?;
    let paths = crate::adapters::path::FsPaths::new(ctx.dirs.root_dir.clone());
    let platform = crate::adapters::platform::StdPlatform;
    use_cmd(client, spec, args.version, &paths, &platform)
        .await
        .map_err(|err| miette::miette!(err))?;

    Ok(())
}

pub async fn use_cmd(
    client: Option<&reqwest::Client>,
    spec: std::sync::Arc<PackageSpec>,
    requested_version: String,
    paths: &crate::adapters::path::FsPaths,
    platform: &impl crate::ports::Platform,
) -> anyhow::Result<()> {
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

    let parsed_version = resolve_requested_version(&requested_version, &spec, &provider).await?;
    let package = Package::with_parsed(spec.clone(), parsed_version.clone(), platform)?;
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
        spec,
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
