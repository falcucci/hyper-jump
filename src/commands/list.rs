use std::fs;
use std::path::PathBuf;

use anyhow::Error;
use anyhow::Result;
use comfy_table::modifiers::UTF8_ROUND_CORNERS;
use comfy_table::presets::UTF8_FULL;
use comfy_table::Cell;
use comfy_table::CellAlignment;
use comfy_table::Color;
use comfy_table::Table;
use tracing::info;

use crate::fs::get_downloads_directory;
use crate::helpers::version::is_version_used;
use crate::packages::Package;
use crate::packages::PackageType;

#[derive(clap::Parser)]
pub struct Args {
    #[command(subcommand)]
    command: Commands,
}

#[derive(clap::Parser)]
pub enum Commands {
    CardanoNode,
    CardanoCli,
    Mithril,
    Aiken,
    Oura,
}

pub async fn run(
    args: Args,
    _ctx: &crate::Context,
    client: Option<&reqwest::Client>,
) -> miette::Result<()> {
    match args.command {
        Commands::Mithril => {
            let package = Package::new(PackageType::Mithril, String::new(), client).await;
            list(package).await.expect("Failed to use")
        }
        Commands::Aiken => {
            let package = Package::new(PackageType::Aiken, String::new(), client).await;
            list(package).await.expect("Failed to use")
        }
        Commands::CardanoNode => {
            let package = Package::new(PackageType::CardanoNode, String::new(), client).await;
            list(package).await.expect("Failed to use")
        }
        Commands::CardanoCli => {
            let package = Package::new(PackageType::CardanoCli, String::new(), client).await;
            list(package).await.expect("Failed to use")
        }
        Commands::Oura => {
            let package = Package::new(PackageType::Oura, String::new(), client).await;
            list(package).await.expect("Failed to use")
        }
    }

    Ok(())
}

pub async fn list(package: Package) -> Result<(), Error> {
    let downloads_dir = get_downloads_directory(package.clone()).await?;

    let paths: Vec<PathBuf> = fs::read_dir(downloads_dir)?
        .filter_map(Result::ok)
        .map(|entry| entry.path())
        .collect();

    if paths.is_empty() {
        info!("There are no versions installed");
        return Ok(());
    }

    let mut table = Table::new();
    let header = vec!["Version", "Status"];
    table.load_preset(UTF8_FULL).apply_modifier(UTF8_ROUND_CORNERS);
    table.set_header(header);

    for path in paths {
        if !path.is_dir() {
            continue;
        }

        let path_name = path.file_name().unwrap().to_str().unwrap();

        let status = if is_version_used(path_name, package.clone()).await {
            Cell::new("Used").fg(Color::Green)
        } else {
            Cell::new("Installed")
        };

        table.add_row(vec![
            Cell::new(path_name).set_alignment(CellAlignment::Center),
            status,
        ]);
    }

    println!("{table}");

    Ok(())
}
