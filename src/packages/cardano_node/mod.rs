use clap::Parser;
use regex::Regex;
use tracing::instrument;

use crate::{
  commands::{
    install::{install, CardanoNode, Package},
    use_cmd::use_cmd,
  },
  helpers::{
    client,
    version::{parse_version_type, ParsedVersion, VersionType},
  },
};

use super::CARDANO_NODE_PACKAGE_URL;

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
      let package = Package::CardanoNode(CardanoNode {
        url: CARDANO_NODE_PACKAGE_URL.to_string(),
        alias: "cardano-node".to_string(),
        version: version.non_parsed_string.clone(),
      });
      println!("Use: {:?}", version);
      use_cmd(&client, version, package)
        .await
        .expect("Failed to set the version")
    }
    Commands::Install { version } => {
      let version = parse_version_type(&version).await.unwrap();
      let cardano_node = Package::CardanoNode(CardanoNode {
        url: CARDANO_NODE_PACKAGE_URL.to_string(),
        alias: "cardano-node".to_string(),
        version: version.non_parsed_string.clone(),
      });

      install(&client, cardano_node, version)
        .await
        .expect("Failed to install")
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
