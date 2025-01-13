use std::fs;
use std::path::PathBuf;

use anyhow::Error;
use anyhow::Result;
use reqwest::Client;
use yansi::Paint;

use crate::helpers::version::is_version_used;
use crate::helpers::version::RemoteVersion;
use crate::helpers::version::VersionStatus;
use crate::packages::Package;
use crate::packages::PackageType;
use crate::services::github::api;
use crate::services::github::deserialize_response;

#[derive(clap::Parser)]
pub struct Args {
    #[command(subcommand)]
    command: Commands,
}

#[derive(clap::Parser)]
pub enum Commands {
    Reth,
    Oura,
    Aiken,
    Dolos,
    Zellij,
    Mithril,
    Scrolls,
    CardanoCli,
    CardanoNode,
    SidechainCli,
    PartnerChainCli,
    PartnerChainNode,
    CardanoSubmitApi,
}

/// Macro to execute a command based on the provided variant and package type.
///
/// This macro matches the provided command against a list of command variants
/// and executes the corresponding code for each variant. It creates a new
/// `Package` instance with the specified package type and client, and then
/// lists the remote package versions.
///
/// # Parameters
///
/// - `$command`: The command to be matched and executed.
/// - `$client`: The client to be used for creating the `Package`.
/// - `$(($variant:ident, $package_type:expr)),*`: A list of tuples containing
///   the command variant and the corresponding package type.
macro_rules! execute {
    ($command:expr, $client:expr, $(($variant:ident, $package_type:expr)),*) => {
        match $command {
            $(
                Commands::$variant => {
                    let package = Package::new($package_type, String::new(), $client).await;
                    list_remote($client, package).await.expect("Failed to list-remote versions")
                }
            )*
        }
    }
}

pub async fn run(
    args: Args,
    _ctx: &crate::Context,
    client: Option<&reqwest::Client>,
) -> miette::Result<()> {
    execute!(
        args.command,
        client,
        (Reth, PackageType::Reth),
        (Oura, PackageType::Oura),
        (Aiken, PackageType::Aiken),
        (Dolos, PackageType::Dolos),
        (Zellij, PackageType::Zellij),
        (Mithril, PackageType::Mithril),
        (Scrolls, PackageType::Scrolls),
        (CardanoCli, PackageType::CardanoCli),
        (CardanoNode, PackageType::CardanoNode),
        (SidechainCli, PackageType::SidechainCli),
        (PartnerChainCli, PackageType::PartnerChainCli),
        (PartnerChainNode, PackageType::PartnerChainNode),
        (CardanoSubmitApi, PackageType::CardanoSubmitApi)
    );

    Ok(())
}

/// Lists the remote versions of a specified package.
///
/// This function fetches the list of releases from a remote repository, filters
/// out pre-release versions, and then prints each version with a specific color
/// coding:
/// - Green if the version is currently used.
/// - Yellow if the version is installed but not used.
/// - Default color if the version is not installed.
///
/// # Arguments
///
/// * `client` - A reference to a `reqwest::Client` used to make HTTP requests.
/// * `package` - The `Package` enum representing the package to list versions
///   for.
///
/// # Returns
///
/// This function returns a `Result<(), Error>` indicating the operation's
/// success or failure.
///
/// # Errors
///
/// This function will return an error if there is no releases URL for the
/// package or if there is an issue with fetching or processing the list of
/// versions.
pub async fn list_remote(client: Option<&Client>, package: Package) -> Result<(), Error> {
    let url = package.releases_url();
    let response = api(client, url).await?;

    let local_versions: Vec<PathBuf> = filter_local_versions(package.clone()).await?;
    let versions: Vec<RemoteVersion> = deserialize_response(response)?;
    let filtered_versions: Vec<RemoteVersion> = filter_versions(versions)?;

    let padding = " ".repeat(12);
    for version in filtered_versions {
        let version_installed = check_version_installed(&local_versions, &version.tag_name);
        let tag = match package {
            Package::CardanoSubmitApi(_) => version.tag_name.clone(),
            Package::PartnerChainNode(_) => version.tag_name.clone(),
            Package::PartnerChainCli(_) => version.tag_name.clone(),
            Package::SidechainCli(_) => version.tag_name.clone(),
            Package::CardanoNode(_) => version.tag_name.clone(),
            Package::CardanoCli(_) => version.tag_name.clone(),
            Package::Mithril(_) => version.tag_name.clone(),
            Package::Scrolls(_) => version.tag_name.clone(),
            Package::Zellij(_) => version.tag_name.clone(),
            Package::Aiken(_) => version.tag_name.clone(),
            Package::Dolos(_) => version.tag_name.clone(),
            Package::Oura(_) => version.tag_name.clone(),
            Package::Reth(_) => version.tag_name.clone(),
        };

        let version_status =
            match is_version_used(format!("v{}", tag).as_str(), package.clone()).await {
                true => VersionStatus::Used,
                false if version_installed => VersionStatus::Installed,
                false => VersionStatus::NotInstalled,
            };

        let color = match version_status {
            VersionStatus::Used => Paint::green(&tag),
            VersionStatus::Installed => {
                retain_local_versions(local_versions.clone(), &version.tag_name);
                Paint::yellow(&tag)
            }
            VersionStatus::NotInstalled => Paint::italic(&tag),
        };

        println!("{padding}{}", color);
    }

    Ok(())
}

fn check_version_installed(local_versions: &[PathBuf], tag: &str) -> bool {
    local_versions.iter().any(|v| {
        v.file_name()
            .and_then(|str| str.to_str())
            .map_or(false, |str| str.contains(tag))
    })
}

fn retain_local_versions(local_versions: Vec<PathBuf>, tag: &str) {
    let mut local_versions = local_versions;
    local_versions.retain(|v| {
        v.file_name()
            .and_then(|str| str.to_str())
            .map_or(true, |str| !str.contains(tag))
    });
}

/// Filters out pre-release versions from a list of `RemoteVersion`.
///
/// # Arguments
///
/// * `versions` - A vector of `RemoteVersion` instances to filter.
///
/// # Returns
///
/// This function returns a `Result<Vec<RemoteVersion>, Error>` containing only
/// the versions that are not marked as pre-releases.
fn filter_versions(versions: Vec<RemoteVersion>) -> Result<Vec<RemoteVersion>, Error> {
    Ok(versions.into_iter().filter(|v| !v.prerelease).collect())
}

/// Filters local versions of a package from the downloads directory.
///
/// This function reads the downloads directory for the specified package and
/// filters out entries that do not contain a 'v' character, which is assumed to
/// indicate a version.
///
/// # Arguments
///
/// * `package` - The `Package` enum representing the package to filter local
///   versions for.
///
/// # Returns
///
/// This function returns a `Result<Vec<PathBuf>, Error>` containing paths to
/// the local versions of the package.
///
/// # Errors
///
/// This function will return an error if there is an issue reading the
/// downloads directory.
async fn filter_local_versions(package: Package) -> Result<Vec<PathBuf>, Error> {
    let downloads_dir = crate::fs::get_downloads_directory(package.clone()).await?;
    let local_versions: Vec<PathBuf> = fs::read_dir(downloads_dir)?
        .filter_map(Result::ok)
        .filter(|entry| entry.path().file_name().unwrap().to_str().unwrap().contains('v'))
        .map(|entry| entry.path())
        .collect();

    Ok(local_versions)
}
