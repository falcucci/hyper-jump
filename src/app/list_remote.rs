use std::path::PathBuf;

use anyhow::Error;
use anyhow::Result;
use serde::Serialize;
use yansi::Paint;

use crate::domain::package::Package;
use crate::domain::version::RemoteVersion;
use crate::domain::version::VersionStatus;
use crate::ports::Fs;
use crate::ports::Output;
use crate::ports::Paths;
use crate::ports::ReleaseProvider;
use crate::ports::UsedVersionStore;
use crate::OutputFormat;

#[derive(Serialize)]
struct RemoteEntry {
    version: String,
    status: String,
}

pub async fn list_remote(
    package: Package,
    fmt: OutputFormat,
    provider: &impl ReleaseProvider,
    output: &impl Output,
    paths: &impl Paths,
    used_store: &impl UsedVersionStore,
    fs: &impl Fs,
) -> Result<(), Error> {
    let versions = provider.list(package.package_type()).await?;
    let downloads_dir = paths.downloads_dir(package.clone()).await?;
    let local_versions: Vec<PathBuf> = filter_local_versions(fs, downloads_dir).await?;
    let filtered_versions: Vec<RemoteVersion> = filter_versions(versions)?;

    let padding = " ".repeat(12);
    let mut json_entries = Vec::new();
    let current = used_store.current(package.clone()).await?;
    let current_norm = current.as_deref().map(normalize_tag);
    for version in filtered_versions {
        let version_installed = check_version_installed(&local_versions, &version.tag_name);
        let tag = match package {
            Package::CardanoSubmitApi(_) => version.tag_name.clone(),
            Package::PartnerChainNode(_) => version.tag_name.clone(),
            Package::PartnerChainCli(_) => version.tag_name.clone(),
            Package::SidechainCli(_) => version.tag_name.clone(),
            Package::CardanoNode(_) => version.tag_name.clone(),
            Package::CardanoCli(_) => version.tag_name.clone(),
            Package::Jujutsu(_) => version.tag_name.clone(),
            Package::Mithril(_) => version.tag_name.clone(),
            Package::Scrolls(_) => version.tag_name.clone(),
            Package::Zellij(_) => version.tag_name.clone(),
            Package::Neovim(_) => version.tag_name.clone(),
            Package::Aiken(_) => version.tag_name.clone(),
            Package::Dolos(_) => version.tag_name.clone(),
            Package::Oura(_) => version.tag_name.clone(),
            Package::Reth(_) => version.tag_name.clone(),
        };

        let tag_norm = normalize_tag(&tag);
        let version_status = match current_norm {
            Some(current) if current == tag_norm => VersionStatus::Used,
            _ if version_installed => VersionStatus::Installed,
            _ => VersionStatus::NotInstalled,
        };

        match fmt {
            OutputFormat::Table => {
                let line = match version_status {
                    VersionStatus::Used => format!("{padding}{}", Paint::green(&tag)),
                    VersionStatus::Installed => {
                        retain_local_versions(local_versions.clone(), &version.tag_name);
                        format!("{padding}{}", Paint::yellow(&tag))
                    }
                    VersionStatus::NotInstalled => format!("{padding}{}", Paint::italic(&tag)),
                };
                output.write_line(&line)?;
            }
            OutputFormat::Json => {
                let status_str = match version_status {
                    VersionStatus::Used => "used",
                    VersionStatus::Installed => "installed",
                    VersionStatus::NotInstalled => "not_installed",
                };
                json_entries.push(RemoteEntry {
                    version: tag.clone(),
                    status: status_str.to_string(),
                });
            }
        }
    }

    if matches!(fmt, OutputFormat::Json) {
        let json = serde_json::to_string_pretty(&json_entries)?;
        output.write_line(&json)?;
    }

    Ok(())
}

fn check_version_installed(local_versions: &[PathBuf], tag: &str) -> bool {
    let tag_norm = normalize_tag(tag);
    local_versions.iter().any(|v| {
        v.file_name()
            .and_then(|str| str.to_str())
            .is_some_and(|str| normalize_tag(str) == tag_norm)
    })
}

fn retain_local_versions(local_versions: Vec<PathBuf>, tag: &str) {
    let mut local_versions = local_versions;
    local_versions.retain(|v| {
        v.file_name().and_then(|str| str.to_str()).is_none_or(|str| !str.contains(tag))
    });
}

fn filter_versions(versions: Vec<RemoteVersion>) -> Result<Vec<RemoteVersion>, Error> {
    Ok(versions.into_iter().filter(|v| !v.prerelease).collect())
}

async fn filter_local_versions(
    fs: &impl Fs,
    downloads_dir: PathBuf,
) -> Result<Vec<PathBuf>, Error> {
    fs.read_dir(&downloads_dir).await
}

fn normalize_tag(tag: &str) -> &str {
    match tag.strip_prefix('v') {
        Some(rest) if rest.chars().next().map(|c| c.is_ascii_digit()).unwrap_or(false) => rest,
        _ => tag,
    }
}
