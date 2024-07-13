use clap::Parser;
use regex::Regex;
use tracing::instrument;

use crate::helpers::version::{ParsedVersion, VersionType};

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
  match args.command {
    Commands::Use { version } => {
      println!("Use: {}", version);
    }
    Commands::Install { version } => {
      let version_regex = Regex::new(r"^v?[0-9]+\.[0-9]+\.[0-9]+$").unwrap();
      if version_regex.is_match(&version) {
        let mut returned_version = version.to_string();
        if !version.contains('v') {
          returned_version.insert(0, 'v');
        }
        let cloned_version = returned_version.clone();
        let mut version = ParsedVersion {
          tag_name: returned_version,
          version_type: VersionType::Normal,
          non_parsed_string: version.to_string(),
          semver: None,
        };

        println!("Install: {:?}", version);
      }
    }
    Commands::Uninstall { version } => {
      println!("Uninstall: {}", version);
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
    Commands::Update(update) => {
      println!("Update: {:?}", update.version);
    }
    Commands::Run(run) => {
      println!("Run: {:?}", run.free);
    }
  }

  Ok(())
}
