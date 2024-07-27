use clap::Parser;
use tracing::instrument;

use crate::commands::install::install;
use crate::commands::install::Package;
use crate::commands::install::PackageType;
use crate::commands::list::list;
use crate::commands::list_remote::list_remote;
use crate::commands::uninstall::uninstall;
use crate::commands::use_cmd::use_cmd;
use crate::helpers::version::VersionType;

#[derive(Parser)]
pub struct Args {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Parser)]
pub enum Commands {
    Use { version: String },
    Install { version: String },
    Uninstall { version: String },
    List,
    ListRemote,
}

#[derive(Parser)]
pub struct Run {
    #[arg(short, long)]
    free: Vec<String>,
}

#[instrument("cardano-node", skip_all)]
pub async fn run(
    args: Args,
    _ctx: &crate::Context,
    client: Option<&reqwest::Client>,
) -> miette::Result<()> {
    match args.command {
        Commands::Use { version } => {
            let package = Package::new(PackageType::CardanoNode, version, client);
            use_cmd(client, package).await.expect("Failed to set the version")
        }
        Commands::Install { version } => {
            let package = Package::new(PackageType::CardanoNode, version, client);
            install(client, package).await.expect("Failed to install")
        }
        Commands::Uninstall { version } => {
            let package = Package::new(PackageType::CardanoNode, version, client);
            uninstall(package).await.expect("Failed to erase");
        }
        Commands::List => {
            let package = Package::new(PackageType::CardanoNode, "".to_string(), client);
            list(package).await.expect("Failed to list");
        }
        Commands::ListRemote => {
            let package = Package::new(PackageType::CardanoNode, "9.0.0".to_string(), client);
            list_remote(client, package).await.expect("Failed to list remote");
        }
    }

    Ok(())
}
