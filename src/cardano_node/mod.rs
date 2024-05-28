use clap::Parser;
use tracing::instrument;

#[derive(Parser)]
pub struct Args {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Parser)]
pub enum Commands {
    Run(Run),
    Bar(Bar),
}

#[derive(Parser)]
pub struct Run {
    #[arg(short, long)]
    free: Vec<String>,
}

#[derive(Parser)]
pub struct Bar {
    #[arg(short, long)]
    free: Vec<String>,
}

#[instrument("cardano-node", skip_all)]
pub async fn run(args: Args, _ctx: &crate::Context) -> miette::Result<()> {
    match args.command {
        Commands::Run(run) => {
            println!("Running run with free: {:?}", run.free);
        }
        Commands::Bar(bar) => {
            println!("Running bar with free: {:?}", bar.free);
        }
    }

    Ok(())
}
