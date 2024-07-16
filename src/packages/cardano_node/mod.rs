use crate::commands::erase::erase;
use crate::commands::install::install;
use crate::commands::install::Package;
use crate::commands::use_cmd::use_cmd;
use crate::helpers::client;
use crate::helpers::version::parse_version_type;
use clap::Parser;
use tracing::instrument;

#[derive(Parser)]
pub struct Args {
    #[command(subcommand)]
    command: Commands,
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
    Update(Update),
    Run(Run),
}

#[derive(Parser)]
pub struct Run {
    #[arg(short, long)]
    free: Vec<String>,
}

#[instrument("cardano-node", skip_all)]
pub async fn run(args: Args, _ctx: &crate::Context) -> miette::Result<()> {
    let client = client::create_reqwest_client().unwrap();
    match args.command {
        Commands::Use { version } => {
            let version = parse_version_type(&version).await.unwrap();
            let package = Package::new_cardano_node(version.non_parsed_string.clone());
            use_cmd(&client, version, package).await.expect("Failed to set the version")
        }
        Commands::Install { version } => {
            let version = parse_version_type(&version).await.unwrap();
            let package = Package::new_cardano_node(version.non_parsed_string.clone());

            install(&client, package, version).await.expect("Failed to install")
        }
        Commands::Uninstall { version } => {
            println!("Uninstall: {}", version);
        }
        Commands::Rollback => {
            println!("Rollback");
        }
        Commands::Erase => {
            let package = Package::new_cardano_node("9.0.0".to_string());
            erase(package).await.expect("Failed to erase");
        }
        Commands::List => {
            println!("List");
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
