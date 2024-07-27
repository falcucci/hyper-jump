use clap::Parser;
use reqwest::Client;
use tracing::instrument;

use crate::commands::install::install;
use crate::commands::install::Package;
use crate::commands::install::PackageType;
use crate::commands::list::list;
use crate::commands::list_remote::list_remote;
use crate::commands::uninstall::uninstall;
use crate::commands::use_cmd::use_cmd;
use crate::helpers::version::VersionType;

#[derive(Parser, Debug)]
pub struct Args {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Parser, Debug)]
pub enum Commands {
    Use { version: String },
    Install { version: String },
    Uninstall { version: String },
    List,
    ListRemote,
}

#[instrument("cardano-cli", skip_all)]
pub async fn run(args: Args, _ctx: &crate::Context, client: Option<&Client>) -> miette::Result<()> {
    match args.command {
        Commands::Use { version } => {
            let package = Package::new(PackageType::CardanoCli, version, client);
            use_cmd(client, package).await.expect("Failed to use")
        }
        Commands::Install { version } => {
            let package = Package::new(PackageType::CardanoCli, version, client);
            install(client, package).await.expect("Failed to install")
        }
        Commands::Uninstall { version } => {
            let package = Package::new(PackageType::CardanoCli, version, client);
            uninstall(package).await.expect("Failed to uninstall")
        }
        Commands::List => {
            let package = Package::new(PackageType::CardanoCli, "".to_string(), client);
            list(package).await.expect("Failed to list");
        }
        Commands::ListRemote => {
            let package = Package::new(PackageType::CardanoCli, "".to_string(), client);
            list_remote(client, package).await.expect("Failed to list remote");
        }
    }

    Ok(())
}
