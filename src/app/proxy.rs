use anyhow::anyhow;
use anyhow::Result;

use crate::domain::package::Package;
use crate::domain::package::PackageRegistry;
use crate::ports::Output;
use crate::ports::Paths;
use crate::ports::Platform;
use crate::ports::Process;
use crate::ports::UsedVersionStore;

pub async fn handle_proxy(
    exec_name: &str,
    rest_args: &[String],
    registry: &PackageRegistry,
    output: &impl Output,
    paths: &impl Paths,
    used_store: &impl UsedVersionStore,
    platform: &impl Platform,
    process: &impl Process,
) -> miette::Result<()> {
    if !rest_args.is_empty() && rest_args[0].eq(concat!("--", env!("CARGO_BIN_NAME"))) {
        output
            .write_line(&format!(
                "{} v{}",
                env!("CARGO_BIN_NAME"),
                env!("CARGO_PKG_VERSION")
            ))
            .map_err(|err| miette::miette!(err))?;
        return Ok(());
    }

    let spec = registry.get_by_alias(exec_name).map_err(|err| miette::miette!(err))?;
    let package = Package::from_spec(spec, platform).map_err(|err| miette::miette!(err))?;

    handle_package_process(rest_args, package, paths, used_store, process)
        .await
        .map_err(|err| miette::miette!("{err}"))?;

    Ok(())
}

pub async fn handle_package_process(
    args: &[String],
    package: Package,
    paths: &impl Paths,
    used_store: &impl UsedVersionStore,
    process: &impl Process,
) -> Result<()> {
    let downloads_dir = paths.downloads_dir(package.clone()).await?;
    let used_version = used_store
        .current(package.clone())
        .await?
        .ok_or_else(|| anyhow!("No version in use for {}", package.alias()))?;

    let location = downloads_dir
        .join(used_version)
        .join(package.binary_path())
        .join(package.binary_name());

    process.run(&location, args).await
}
