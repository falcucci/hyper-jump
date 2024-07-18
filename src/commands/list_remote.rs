use std::fs;
use std::path::PathBuf;

use anyhow::anyhow;
use anyhow::Error;
use anyhow::Result;
use reqwest::Client;
use yansi::Paint;

use super::install::Package;
use crate::helpers::version::is_version_used;
use crate::helpers::version::RemoteVersion;
use crate::services::github::api;
use crate::services::github::deserialize_response;

pub async fn list_remote(client: &Client, package: Package) -> Result<(), Error> {
    let response = api(
        client,
        package.clone().releases_url().ok_or(anyhow!("No releases URL"))?,
    )
    .await?;

    let mut local_versions: Vec<PathBuf> = filter_local_versions(package.clone()).await?;
    let versions: Vec<RemoteVersion> = deserialize_response(response)?;
    let filtered_versions: Vec<RemoteVersion> = filter_versions(versions)?;

    let padding = " ".repeat(12);
    for version in filtered_versions {
        let version_installed = local_versions.iter().any(|v| {
            v.file_name()
                .and_then(|str| str.to_str())
                .map_or(false, |str| str.contains(&version.tag_name))
        });

        let tag = match package {
            Package::CardanoNode(_) => version.tag_name.clone(),
            Package::CardanoCli(_) => version.name.clone(),
            Package::Mithril => todo!(),
        };

        if is_version_used(format!("v{}", tag).as_str(), package.clone()).await {
            println!("{padding}{}", Paint::green(&tag),);
        } else if version_installed {
            println!("{padding}{}", Paint::yellow(&tag),);

            local_versions.retain(|v| {
                v.file_name()
                    .and_then(|str| str.to_str())
                    .map_or(true, |str| !str.contains(&version.name))
            });
        } else {
            println!("{padding}{}", tag);
        }
    }

    Ok(())
}

fn filter_versions(versions: Vec<RemoteVersion>) -> Result<Vec<RemoteVersion>, Error> {
    Ok(versions.into_iter().filter(|v| !v.prerelease).collect())
}

async fn filter_local_versions(package: Package) -> Result<Vec<PathBuf>, Error> {
    let downloads_dir = crate::fs::get_downloads_directory(package.clone()).await?;
    let local_versions: Vec<PathBuf> = fs::read_dir(downloads_dir)?
        .filter_map(Result::ok)
        .filter(|entry| entry.path().file_name().unwrap().to_str().unwrap().contains('v'))
        .map(|entry| entry.path())
        .collect();

    Ok(local_versions)
}
