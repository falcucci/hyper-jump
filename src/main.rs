use std::{
    env,
    path::{Path, PathBuf},
};

extern crate core;

use clap::{Parser, Subcommand, ValueEnum};
use commands::install::{CardanoNode, Package};
use packages::{
    cardano_cli,
    cardano_node::{self, processes::handle_cardano_node_process},
    mithril, CARDANO_NODE_PACKAGE_URL,
};
use tracing::Level;
use tracing_indicatif::IndicatifLayer;
use tracing_subscriber::{layer::SubscriberExt, util::SubscriberInitExt};

mod commands;
mod dirs;
mod fs;
mod helpers;
mod packages;

#[derive(Parser)]
#[command(author, version, about, long_about = None)]
struct Cli {
    #[command(subcommand)]
    command: Commands,

    #[arg(
        short,
        long,
        global = true,
        help = "root dir for config and data",
        env = "Hyper-jump_ROOT_DIR"
    )]
    root_dir: Option<PathBuf>,

    #[arg(
        short,
        long,
        global = true,
        help = "output format for command response",
        env = "Hyper-jump_OUTPUT_FORMAT"
    )]
    output_format: Option<OutputFormat>,
}

#[derive(ValueEnum, Clone)]
pub enum OutputFormat {
    Json,
    Table,
}

#[derive(Subcommand)]
enum Commands {
    Mithril(mithril::Args),
    CardanoNode(cardano_node::Args),
    CardanoCli(cardano_cli::Args),
}

pub struct Context {
    pub dirs: dirs::Dirs,
    pub output_format: OutputFormat,
}

impl Context {
    fn for_cli(cli: &Cli) -> miette::Result<Self> {
        let dirs = dirs::Dirs::try_new(cli.root_dir.as_deref())?;
        let output_format = cli.output_format.clone().unwrap_or(OutputFormat::Table);

        Ok(Context {
            dirs,
            output_format,
        })
    }
}

pub fn with_tracing() {
    let indicatif_layer = IndicatifLayer::new();
    tracing_subscriber::registry()
        .with(tracing_subscriber::filter::LevelFilter::INFO)
        .with(tracing_subscriber::filter::Targets::default().with_target("hyper-jump", Level::INFO))
        .with(tracing_subscriber::fmt::layer().with_writer(indicatif_layer.get_stderr_writer()))
        .with(indicatif_layer)
        .init();
}

#[tokio::main]
async fn main() -> miette::Result<()> {
    let args: Vec<String> = env::args().collect();

    let exe_name_path = Path::new(&args[0]);
    let exe_name = exe_name_path.file_stem().unwrap().to_str().unwrap();

    let rest_args = &args[1..];

    if !exe_name.eq("hyper-jump") {
        if !rest_args.is_empty() && rest_args[0].eq("--hyper-jump") {
            print!("hyper-jump v{}", env!("CARGO_PKG_VERSION"));
            return Ok(());
        }

        let package = Package::CardanoNode(CardanoNode {
            url: CARDANO_NODE_PACKAGE_URL.to_string(),
            alias: "cardano-node".to_string(),
            version: "9.0.0".to_string(),
        });

        handle_cardano_node_process(rest_args, package)
            .await
            .unwrap();

        return Ok(());
    }

    let cli = Cli::parse();

    let ctx = Context::for_cli(&cli)?;

    match cli.command {
        Commands::Mithril(args) => mithril::run(args, &ctx).await,
        Commands::CardanoNode(args) => cardano_node::run(args, &ctx).await,
        Commands::CardanoCli(args) => cardano_cli::run(args, &ctx).await,
    }
}
