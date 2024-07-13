use clap::Parser;
use regex::Regex;
use semver::Version;
use tracing::instrument;

/// TODO: move this to a version mod.
/// Represents a parsed version of the software.
///
/// This struct contains information about a parsed version of the software, including the tag name, version type, non-parsed string, and semantic version.
///
/// # Fields
///
/// * `tag_name: String` - The tag name of the parsed version.
/// * `version_type: VersionType` - The type of the parsed version.
/// * `non_parsed_string: String` - The non-parsed string of the parsed version.
/// * `semver: Option<Version>` - The semantic version of the parsed version, or `None` if the version is not a semantic version.
///
/// # Example
///
/// ```rust
/// let parsed_version = ParsedVersion {
///     tag_name: "v1.0.0".to_string(),
///     version_type: VersionType::Normal,
///     non_parsed_string: "version-1.0.0".to_string(),
///     semver: Some(Version::parse("1.0.0").unwrap()),
/// };
/// println!("The parsed version is {:?}", parsed_version);
/// ```
#[derive(Debug)]
pub struct ParsedVersion {
  pub tag_name: String,
  pub version_type: VersionType,
  pub non_parsed_string: String,
  pub semver: Option<Version>,
}

/// TODO: move this to a version mod.
/// Represents the type of a software version.
///
/// This enum is used to distinguish between different types of software versions, such as normal versions, the latest version, nightly versions, versions identified by a hash, and nightly versions that have been rolled back.
///
/// # Variants
///
/// * `Normal` - Represents a normal version.
/// * `Latest` - Represents the latest version.
/// * `Hash` - Represents a version identified by a hash.
///
/// # Example
///
/// ```rust
/// let version_type = VersionType::Nightly;
/// match version_type {
///     VersionType::Normal => println!("This is a normal version."),
///     VersionType::Latest => println!("This is the latest version."),
///     VersionType::Hash => println!("This is a version identified by a hash."),
/// }
/// ```
#[derive(PartialEq, Eq, Debug)]
pub enum VersionType {
  Normal,
  Latest,
  Hash,
}

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
