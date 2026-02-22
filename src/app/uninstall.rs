use anyhow::Error;
use anyhow::Result;
use tracing::info;

use crate::app::resolve::resolve_requested_version;
use crate::domain::package::Package;
use crate::domain::package::PackageSpec;
use crate::ports::Fs;
use crate::ports::Paths;
use crate::ports::Platform;
use crate::ports::ReleaseProvider;
use crate::ports::UsedVersionStore;

pub async fn uninstall_requested(
    spec: std::sync::Arc<PackageSpec>,
    requested_version: String,
    release_provider: &impl ReleaseProvider,
    platform: &impl Platform,
    fs: &impl Fs,
    paths: &impl Paths,
    used_store: &impl UsedVersionStore,
) -> Result<(), Error> {
    let parsed = resolve_requested_version(&requested_version, &spec, release_provider).await?;
    let package = Package::with_parsed(spec, parsed, platform)?;
    uninstall(package, fs, paths, used_store).await
}

pub async fn uninstall(
    package: Package,
    fs: &impl Fs,
    paths: &impl Paths,
    used_store: &impl UsedVersionStore,
) -> Result<(), Error> {
    let parsed_version = package.version().expect("Failed to parse version");
    let version = parsed_version.non_parsed_string.clone();
    let used_version = used_store.current(package.clone()).await?.unwrap_or_default();
    let same_version = used_version == version;

    let mut downloads = paths.downloads_dir(package.clone()).await?;
    let location = downloads.join("used");
    downloads.push(&version);

    if fs.remove_dir_all(&downloads).await.is_ok() {
        info!("Successfully uninstalled {} installation", &version);
    } else {
        info!("There's nothing to uninstall");
    }

    if !same_version {
        return Ok(());
    }

    if fs.remove_file(&location).await.is_ok() {
        info!("Successfully removed {} from used versions", &version);
    } else {
        info!("There's nothing to uninstall");
    }

    Ok(())
}
