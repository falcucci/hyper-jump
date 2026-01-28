use std::path::Path;
use std::path::PathBuf;

use tracing::info;

use crate::app::download;
use crate::app::layout;
use crate::app::resolve::resolve_requested_version;
use crate::domain::package::Package;
use crate::domain::package::PackageType;
use crate::domain::version::LocalVersion;
use crate::domain::version::ParsedVersion;
use crate::ports::Archive;
use crate::ports::Downloader;
use crate::ports::Fs;
use crate::ports::Paths;
use crate::ports::Platform;
use crate::ports::ProxyInstaller;
use crate::ports::ReleaseProvider;

#[allow(clippy::too_many_arguments)]
pub async fn install<R, D, A, F>(
    package_type: PackageType,
    requested_version: String,
    release_provider: &R,
    downloader: &D,
    archive: &A,
    fs: &F,
    platform: &impl Platform,
    lock: &impl crate::ports::Lock,
    used_store: &impl crate::ports::UsedVersionStore,
    paths: &impl Paths,
    proxy: &impl ProxyInstaller,
) -> anyhow::Result<()>
where
    R: ReleaseProvider,
    D: Downloader,
    A: Archive,
    F: Fs,
{
    let _guard = lock.acquire().await?;
    let parsed_version: ParsedVersion =
        resolve_requested_version(&requested_version, package_type.clone(), release_provider)
            .await?;

    let binary_path = layout::binary_path(package_type.clone(), platform);
    let package = Package::with_parsed(package_type.clone(), parsed_version.clone(), binary_path);
    let root: PathBuf = paths.downloads_dir(package.clone()).await?;
    fs.ensure_dir(&root).await?;
    fs.set_current_dir(&root).await?;

    if version_exists(fs, &parsed_version.tag_name, &root).await? {
        info!("{} is already installed.", parsed_version.tag_name);
        proxy.ensure_proxy(&package.alias()).await?;
        return Ok(());
    }

    proxy.ensure_proxy(&package.alias()).await?;

    // If nothing marked as used yet, set this one.
    if used_store.current(package.clone()).await?.is_none() {
        used_store.set_current(package.clone(), &parsed_version.tag_name).await?;
    }

    let file_type = platform.file_type(package_type.clone());
    let file_path = root.join(format!("{}.{}", parsed_version.tag_name, file_type));
    let download_url = download::download_url(&package, platform);
    downloader.download(&download_url, &file_path).await?;

    let local_version = LocalVersion {
        file_name: parsed_version.tag_name.to_owned(),
        file_format: file_type.to_string(),
        path: root.display().to_string(),
        semver: parsed_version.semver.clone(),
    };

    archive.extract(package, local_version).await?;
    info!("Successfully installed {}", parsed_version.tag_name);

    Ok(())
}

async fn version_exists(fs: &impl Fs, version: &str, downloads_dir: &Path) -> anyhow::Result<bool> {
    let entries = fs.read_dir(downloads_dir).await?;
    for entry in entries {
        if !fs.is_dir(&entry).await? {
            continue;
        }
        let name = entry.file_name().unwrap_or_default().to_string_lossy().to_string();
        if name == version {
            return Ok(true);
        }
    }
    Ok(false)
}
