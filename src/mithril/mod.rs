use clap::{command, Parser};
use tracing::instrument;

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
    Use {
        /// Version to switch to |nightly|stable|<version-string>|<commit-hash>|
        version: String,
    },

    Install {
        /// Version to install |nightly|stable|<version-string>|<commit-hash>|
        version: String,
    },

    Uninstall {
        /// Version to uninstall |nightly|stable|<version-string>|<commit-hash>|
        version: String,
    },

    Rollback,

    // Erase any change hyper-jump has made to the system
    Erase,

    // List all installed versions
    List,

    Update(Update),
    Run(Run),
}

#[instrument("mithril", skip_all)]
pub async fn run(args: Args, _ctx: &crate::Context) -> miette::Result<()> {
    match args.command {
        Commands::Use { version } => {
            println!("Running use with version: {}", version);
        }
        Commands::Install { version } => {
            println!("Running install with version: {}", version);
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
        Commands::Update(update) => {
            println!("Running update with version: {:?}", update.version);
        }
        Commands::Run(run) => {
            println!("Running run with free: {:?}", run.free);
        }
    }

    Ok(())
}
