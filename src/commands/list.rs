use crate::adapters::fs::TokioFs;
use crate::adapters::output::StdoutOutput;
use crate::app::list::list_installed;
use crate::domain::package::Package;

#[derive(clap::Parser)]
pub struct Args {
    pub package: String,
}

pub async fn run(
    args: Args,
    ctx: &crate::Context,
    _client: Option<&reqwest::Client>,
) -> miette::Result<()> {
    let spec = ctx.packages.resolve(&args.package).map_err(|e| miette::miette!(e))?;
    let output = StdoutOutput;
    let fs = TokioFs;
    let platform = crate::adapters::platform::StdPlatform;
    let paths = crate::adapters::path::FsPaths::new(ctx.dirs.root_dir.clone());
    let used_store = crate::adapters::used_store::UsedFileStore::new(paths.clone());
    let package = Package::from_spec(spec, &platform).map_err(|e| miette::miette!(e))?;
    list_installed(
        package,
        ctx.output_format.clone(),
        &output,
        &paths,
        &used_store,
        &fs,
    )
    .await
    .map_err(|e| miette::miette!(e))
}
