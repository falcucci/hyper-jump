use tracing::info;

use crate::commands::install::install;
use crate::fs::copy_package_proxy;
use crate::helpers::version::is_version_used;
use crate::helpers::version::switch_version;
use crate::packages::Package;
use crate::packages::PackageType;

macro_rules! execute {
  ($command:expr, $client:expr, $(($variant:ident, $package_type:expr)),*) => {
    match $command {
      $(
        Commands::$variant { version } => {
          let package = Package::new($package_type, version, $client).await;
          use_cmd($client, package).await.expect("Failed to use");
        }
      )*
    }
  }
}

#[derive(clap::Parser)]
pub struct Args {
    #[command(subcommand)]
    command: Commands,
}

#[derive(clap::Parser)]
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
