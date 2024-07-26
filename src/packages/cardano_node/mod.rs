use clap::Parser;
use tracing::instrument;

use crate::commands::install::install;
use crate::commands::install::Package;
use crate::commands::install::PackageType;
use crate::commands::list::list;
use crate::commands::list_remote::list_remote;
use crate::commands::uninstall::uninstall;
use crate::commands::use_cmd::use_cmd;
use crate::helpers::client;
use crate::helpers::version::parse_version_type;

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
    client: &reqwest::Client,
) -> miette::Result<()> {
    match args.command {
        Commands::Use { version } => {
            let version = parse_version_type(&version).await.unwrap();
            let package = Package::new(PackageType::CardanoNode, version.non_parsed_string.clone());
            use_cmd(client, version, package).await.expect("Failed to set the version")
        }
        Commands::Install { version } => {
            let version = parse_version_type(&version).await.unwrap();
            let package = Package::new(PackageType::CardanoNode, version.non_parsed_string.clone());
            install(client, package, version).await.expect("Failed to install")
        }
        Commands::Uninstall { version } => {
            let version = parse_version_type(&version).await.unwrap();
            let package = Package::new(PackageType::CardanoNode, version.non_parsed_string.clone());
            uninstall(package).await.expect("Failed to erase");
        }
        Commands::List => {
            let package = Package::new(PackageType::CardanoNode, "".to_string());
            list(package).await.expect("Failed to list");
        }
        Commands::ListRemote => {
            let package = Package::new(PackageType::CardanoNode, "9.0.0".to_string());
            list_remote(client, package).await.expect("Failed to list remote");
        }
    }

    Ok(())
}
