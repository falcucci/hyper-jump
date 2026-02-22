mod adapters;
mod app;
mod commands;
mod domain;
mod ports;

use std::path::PathBuf;

use adapters::client;
use adapters::env::StdEnv;
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
use tracing::Level;
use tracing_indicatif::IndicatifLayer;
use tracing_subscriber::layer::SubscriberExt;
use tracing_subscriber::util::SubscriberInitExt;

extern crate core;

#[derive(Parser)]
#[command(author, version, about, long_about = None, name = "hj")]
struct Cli {
    #[command(subcommand)]
    command: Commands,

    #[arg(
        short,
        long,
        global = true,
        help = "root dir for config and data",
        env = "HYPER_JUMP_ROOT_DIR"
    )]
    root_dir: Option<PathBuf>,

    #[arg(
        short,
        long,
        global = true,
        help = "output format for command response",
        env = "HYPER_JUMP_OUTPUT_FORMAT"
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
    pub dirs: adapters::dirs::Dirs,
    pub output_format: OutputFormat,
}

impl Context {
    fn for_cli(cli: &Cli, env: &dyn crate::ports::Env) -> miette::Result<Self> {
        let dirs = adapters::dirs::Dirs::try_new(cli.root_dir.as_deref(), env)?;
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
    tracing_subscriber::fmt::init();
    let env_adapter = StdEnv;
    let env_ref: &dyn crate::ports::Env = &env_adapter;
    let args: Vec<String> = env_ref.args();
    let exe_name = env_ref.exe_name();
    let rest_args = args[1..].to_vec();

    if !exe_name.eq(env!("CARGO_BIN_NAME")) {
        let root_dir = env_ref.root_dir();
        let dirs = adapters::dirs::Dirs::try_new(root_dir.as_deref(), env_ref)?;
        let paths = adapters::path::FsPaths::new(dirs.root_dir.clone());
        let used_store = adapters::used_store::UsedFileStore::new(paths.clone());
        let platform = adapters::platform::StdPlatform;
        let process = adapters::process::TokioProcess;
        let output = adapters::output::StdoutOutput;
        return app::proxy::handle_proxy(
            &exe_name,
            &rest_args,
            &output,
            &paths,
            &used_store,
            &platform,
            &process,
        )
        .await;
    }

    let cli = Cli::parse();
    let ctx = Context::for_cli(&cli, env_ref)?;
    let client = Some(client::create_reqwest_client().map_err(|e| miette::miette!(e))?);

    match cli.command {
        Commands::Use(args) => use_cmd::run(args, &ctx, client.as_ref()).await,
        Commands::List(args) => list::run(args, &ctx, client.as_ref()).await,
        Commands::Install(args) => install::run(args, &ctx, client.as_ref()).await,
        Commands::Uninstall(args) => uninstall::run(args, &ctx, client.as_ref()).await,
        Commands::ListRemote(args) => list_remote::run(args, &ctx, client.as_ref()).await,
        Commands::Prefix => prefix::run(&ctx).await,
        Commands::Erase => erase::run(&ctx).await,
    }
}
