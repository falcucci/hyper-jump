use clap::Parser;
use reqwest::Client;
use tracing::instrument;

use crate::commands::install::install;
use crate::commands::install::Package;
use crate::commands::list_remote::list_remote;
use crate::commands::use_cmd::use_cmd;
use crate::helpers::client;
use crate::helpers::version::parse_version_type;

#[derive(Parser)]
pub struct Args {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Parser)]
pub struct Run {
    #[arg(short, long)]
    free: Vec<String>,
}

#[derive(Parser)]
pub struct Update {
    /// Update specified version |nightly|stable|
    #[arg(conflicts_with = "all")]
    pub version: Option<String>,

    /// Apply the update to all versions
    #[arg(short, long)]
    pub all: bool,

    #[arg(short, long)]
    force: bool,
}

#[derive(Parser)]
pub enum Commands {
    Use { version: String },
    Install { version: String },
    Uninstall { version: String },
    Rollback,
    Erase,
    List,
    ListRemote,
    Update(Update),
    Run(Run),
}

#[instrument("cardano-cli", skip_all)]
pub async fn run(args: Args, _ctx: &crate::Context, client: &Client) -> miette::Result<()> {
    match args.command {
        Commands::Use { version } => {
            let version = parse_version_type(&version).await.unwrap();
            let package = Package::new_cardano_cli(version.non_parsed_string.clone());
            use_cmd(client, version, package).await.expect("Failed to use")
        }
        Commands::Install { version } => {
            let version = parse_version_type(&version).await.unwrap();
            let package = Package::new_cardano_cli("9.0.0".to_string());
            install(client, package, version).await.expect("Failed to install")
        }
        Commands::Uninstall { version } => {
            println!("Uninstall version: {}", version);
        }
        Commands::Rollback => {
            println!("Rollback");
        }
        Commands::Erase => {
            println!("Erase");
        }
        Commands::List => {
            println!("List");
        }
        Commands::ListRemote => {
            let package = Package::new_cardano_cli("9.0.0.1".to_string());
            list_remote(client, package).await.expect("Failed to list remote");
        }
        Commands::Update(update) => {
            println!("Update: {:?}", update.version);
        }
        Commands::Run(run) => {
            println!("Run: {:?}", run.free);
        }
    }

    Ok(())
}
