use clap::command;
use clap::Parser;
use tracing::instrument;

use crate::commands::install::install;
use crate::commands::install::Package;
use crate::commands::list_remote::list_remote;
use crate::commands::use_cmd::use_cmd;
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
    List,
    ListRemote,
    Erase,
    Update(Update),
    Run(Run),
}

#[instrument("mithril", skip_all)]
pub async fn run(
    args: Args,
    _ctx: &crate::Context,
    client: &reqwest::Client,
) -> miette::Result<()> {
    match args.command {
        Commands::Use { version } => {
            let version = parse_version_type(version.as_str()).await.unwrap();
            let package = Package::new_mithril(version.non_parsed_string.clone());
            use_cmd(client, version, package).await.expect("Failed to use")
        }
        Commands::Install { version } => {
            let version = parse_version_type(version.as_str()).await.unwrap();
            let package = Package::new_mithril(version.non_parsed_string.clone());
            install(client, package, version).await.expect("Failed to install")
        }
        Commands::Uninstall { version } => {
            println!("Running uninstall with version: {}", version);
        }
        Commands::Rollback => {
            println!("Running rollback");
        }
        Commands::Erase => {
            println!("Running erase");
        }
        Commands::List => {
            println!("Running list");
        }
        Commands::ListRemote => {
            let package = Package::new_mithril("9.0.0".to_string());
            list_remote(client, package).await.expect("Failed to list remote");
        }
        Commands::Update(update) => {
            println!("Running update with version: {:?}", update.version);
        }
        Commands::Run(run) => {
            println!("Running run with free: {:?}", run.free);
        }
    }

    Ok(())
}
