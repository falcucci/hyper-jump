use clap::Parser;

use crate::adapters::archive::LocalArchive;
use crate::adapters::downloader::ReqwestDownloader;
use crate::adapters::fs::TokioFs;
use crate::adapters::github_release::GitHubReleaseProvider;
use crate::app::install as app_install;

#[derive(Parser)]
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

    app_install::install(
        spec,
        args.version,
        &provider,
        &downloader,
        &archive,
        &fs,
        &platform,
        &lock,
        &used_store,
        &paths,
        &proxy,
    )
    .await
    .map_err(|e| miette::miette!(e))
}
