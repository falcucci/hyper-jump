use std::path::PathBuf;

use clap::{Parser, Subcommand, ValueEnum};
use tracing::Level;
use tracing_indicatif::IndicatifLayer;
use tracing_subscriber::{layer::SubscriberExt, util::SubscriberInitExt};

mod cardano_cli;
mod cardano_node;
mod dirs;
mod mithril;

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
    let cli = Cli::parse();

    let ctx = Context::for_cli(&cli)?;

    match cli.command {
        Commands::Mithril(args) => mithril::run(args, &ctx).await,
        Commands::CardanoNode(args) => cardano_node::run(args, &ctx).await,
        Commands::CardanoCli(args) => cardano_cli::run(args, &ctx).await,
    }
}
