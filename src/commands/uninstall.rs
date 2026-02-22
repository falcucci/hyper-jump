use clap::Parser;

use crate::adapters::fs::TokioFs;
use crate::adapters::github_release::GitHubReleaseProvider;
use crate::adapters::path::FsPaths;
use crate::adapters::used_store::UsedFileStore;
use crate::app::uninstall::uninstall_requested;

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
    let paths = FsPaths::new(ctx.dirs.root_dir.clone());
    let used_store = UsedFileStore::new(paths.clone());
    let fs = TokioFs;
    let platform = crate::adapters::platform::StdPlatform;
    let provider = GitHubReleaseProvider::new(client);

    uninstall_requested(
        spec,
        args.version,
        &provider,
        &platform,
        &fs,
        &paths,
        &used_store,
    )
    .await
    .map_err(|e| miette::miette!(e))
}
