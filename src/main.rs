mod commands;
mod dirs;
mod fs;
mod helpers;
mod packages;
mod proxy;
mod services;

use std::env;
use std::path::Path;
use std::path::PathBuf;

use clap::Parser;
use clap::Subcommand;
use clap::ValueEnum;
use commands::erase;
use commands::install;
use commands::list;
use commands::list_remote;
use commands::prefix;
use commands::uninstall;
use commands::use_cmd;
use helpers::client;
use proxy::handle_proxy;
use tracing::Level;
use tracing_indicatif::IndicatifLayer;
use tracing_subscriber::layer::SubscriberExt;
use tracing_subscriber::util::SubscriberInitExt;

extern crate core;

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
    Use(use_cmd::Args),
    List(list::Args),
    Install(install::Args),
    Uninstall(uninstall::Args),
    ListRemote(list_remote::Args),
    Prefix,
    Erase,
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

fn parse_args(args: Vec<String>) -> (String, Vec<String>) {
    let exe_name_path = Path::new(&args[0]);
    let exe_name = exe_name_path.file_stem().unwrap().to_str().unwrap();
    let rest_args = &args[1..];
    (exe_name.to_string(), rest_args.to_vec())
}

#[tokio::main]
async fn main() -> miette::Result<()> {
    tracing_subscriber::fmt::init();
    let args: Vec<String> = env::args().collect();
    let (exe_name, rest_args) = parse_args(args);

    if !exe_name.eq(env!("CARGO_PKG_NAME")) {
        return handle_proxy(&exe_name, &rest_args).await;
    }

    let cli = Cli::parse();
    let ctx = Context::for_cli(&cli)?;
    let client = Some(client::create_reqwest_client().map_err(|e| miette::miette!(e))?);

    match cli.command {
        Commands::Use(args) => use_cmd::run(args, &ctx, client.as_ref()).await,
        Commands::List(args) => list::run(args, &ctx, client.as_ref()).await,
        Commands::Install(args) => install::run(args, &ctx, client.as_ref()).await,
        Commands::Uninstall(args) => uninstall::run(args, &ctx, client.as_ref()).await,
        Commands::ListRemote(args) => list_remote::run(args, &ctx, client.as_ref()).await,
        Commands::Prefix => prefix::run().await,
        Commands::Erase => erase::run().await,
    }
}
