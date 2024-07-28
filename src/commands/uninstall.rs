use anyhow::Error;
use anyhow::Result;
use clap::Parser;
use tokio::fs;
use tracing::info;

use crate::fs::get_downloads_directory;
use crate::helpers::version::get_current_version;
use crate::packages::Package;
use crate::packages::PackageType;

#[derive(Parser)]
pub struct Args {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Parser)]
pub enum Commands {
    CardanoNode { version: String },
    CardanoCli { version: String },
    Mithril { version: String },
    Aiken { version: String },
}

pub async fn run(
    args: Args,
    _ctx: &crate::Context,
    client: Option<&reqwest::Client>,
) -> miette::Result<()> {
    match args.command {
        Commands::Mithril { version } => {
            let package = Package::new(PackageType::Mithril, version, client).await;
            uninstall(package).await.expect("Failed to uninstall")
        }
        Commands::Aiken { version } => {
            let package = Package::new(PackageType::Aiken, version, client).await;
            uninstall(package).await.expect("Failed to uninstall")
        }
        Commands::CardanoNode { version } => {
            let package = Package::new(PackageType::CardanoNode, version, client).await;
            uninstall(package).await.expect("Failed to uninstall")
        }
        Commands::CardanoCli { version } => {
            let package = Package::new(PackageType::CardanoCli, version, client).await;
            uninstall(package).await.expect("Failed to uninstall")
        }
    }

    Ok(())
}

pub async fn uninstall(package: Package) -> Result<(), Error> {
    let parsed_version = package.version().expect("Failed to parse version");
    let version = parsed_version.non_parsed_string.clone();
    let used_version = get_current_version(package.clone()).await?;
    let same_version = used_version == version;

    let mut downloads = get_downloads_directory(package.clone()).await?;
    let location = downloads.join("used");
    downloads.push(&version);

    if fs::remove_dir_all(&downloads).await.is_ok() {
        info!("Successfully uninstalled {} installation", &version);
    } else {
        info!("There's nothing to uninstall");
    }

    if !same_version {
        return Ok(());
    }

    if fs::remove_file(location).await.is_ok() {
        info!("Successfully removed {} from used versions", &version);
    } else {
        info!("There's nothing to uninstall");
    }

    Ok(())
}
