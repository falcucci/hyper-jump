use anyhow::Error;
use anyhow::Result;
use clap::Parser;
use tokio::fs;
use tracing::info;

use crate::fs::get_downloads_directory;
use crate::helpers::version::get_current_version;
use crate::packages::Package;
use crate::packages::PackageType;

macro_rules! execute {
    ($command:expr, $client:expr, $(($variant:ident, $package_type:expr)),*) => {
        match $command {
            $(
                Commands::$variant { version } => {
                    let package = Package::new($package_type, version, $client).await;
                    uninstall(package).await.expect("Failed to uninstall")
                }
            )*
        }
    }
}

#[derive(Parser)]
pub struct Args {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Parser)]
pub enum Commands {
    Oura { version: String },
    Aiken { version: String },
    Dolos { version: String },
    Mithril { version: String },
    Scrolls { version: String },
    CardanoCli { version: String },
    CardanoNode { version: String },
}

pub async fn run(
    args: Args,
    _ctx: &crate::Context,
    client: Option<&reqwest::Client>,
) -> miette::Result<()> {
    execute!(
        args.command,
        client,
        (Oura, PackageType::Oura),
        (Aiken, PackageType::Aiken),
        (Dolos, PackageType::Dolos),
        (Mithril, PackageType::Mithril),
        (Scrolls, PackageType::Scrolls),
        (CardanoCli, PackageType::CardanoCli),
        (CardanoNode, PackageType::CardanoNode)
    );

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
