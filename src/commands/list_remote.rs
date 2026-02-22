use crate::adapters::fs::TokioFs;
use crate::adapters::github_release::GitHubReleaseProvider;
use crate::adapters::output::StdoutOutput;
use crate::app::list_remote::list_remote as app_list_remote;
use crate::domain::package::Package;

#[derive(clap::Parser)]
pub struct Args {
    pub package: String,
}

pub async fn run(
    args: Args,
    ctx: &crate::Context,
    client: Option<&reqwest::Client>,
) -> miette::Result<()> {
    let spec = ctx.packages.resolve(&args.package).map_err(|e| miette::miette!(e))?;
    let platform = crate::adapters::platform::StdPlatform;
    let package = Package::from_spec(spec, &platform).map_err(|e| miette::miette!(e))?;
    let provider = GitHubReleaseProvider::new(client);
    let output = StdoutOutput;
    let paths = crate::adapters::path::FsPaths::new(ctx.dirs.root_dir.clone());
    let used_store = crate::adapters::used_store::UsedFileStore::new(paths.clone());
    let fs = TokioFs;

    app_list_remote(
        package,
        ctx.output_format.clone(),
        &provider,
        &output,
        &paths,
        &used_store,
        &fs,
    )
    .await
    .map_err(|e| miette::miette!(e))
}
