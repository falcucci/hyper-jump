use tracing::info;

use crate::commands::install::install;
use crate::fs::copy_package_proxy;
use crate::helpers::version::is_version_used;
use crate::helpers::version::switch_version;
use crate::packages::Package;
use crate::packages::PackageType;

#[derive(clap::Parser)]
pub struct Args {
    #[command(subcommand)]
    command: Commands,
}

#[derive(clap::Parser)]
pub enum Commands {
    CardanoNode { version: String },
    CardanoCli { version: String },
    Mithril { version: String },
    Aiken { version: String },
    Oura { version: String },
}

pub async fn run(
    args: Args,
    _ctx: &crate::Context,
    client: Option<&reqwest::Client>,
) -> miette::Result<()> {
    match args.command {
        Commands::Mithril { version } => {
            let package = Package::new(PackageType::Mithril, version, client).await;
            use_cmd(client, package).await.expect("Failed to use")
        }
        Commands::Aiken { version } => {
            let package = Package::new(PackageType::Aiken, version, client).await;
            use_cmd(client, package).await.expect("Failed to use")
        }
        Commands::Oura { version } => {
            let package = Package::new(PackageType::Oura, version, client).await;
            use_cmd(client, package).await.expect("Failed to use")
        }
        Commands::CardanoNode { version } => {
            let package = Package::new(PackageType::CardanoNode, version, client).await;
            use_cmd(client, package).await.expect("Failed to use")
        }
        Commands::CardanoCli { version } => {
            let package = Package::new(PackageType::CardanoCli, version, client).await;
            use_cmd(client, package).await.expect("Failed to use")
        }
    }

    Ok(())
}

pub async fn use_cmd(
    client: Option<&reqwest::Client>,
    package: Package,
) -> Result<(), Box<dyn std::error::Error>> {
    let version = package.version().unwrap();
    let is_version_used = is_version_used(&version.tag_name, package.clone()).await;

    copy_package_proxy(package.clone()).await?;

    if is_version_used {
        info!("{} is already in use.", version.tag_name);
        return Ok(());
    }

    install(client, package.clone()).await?;

    switch_version(&version, package.clone()).await?;

    info!("You can now use {}!", version.tag_name);

    Ok(())
}
