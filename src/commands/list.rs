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
    Reth,
    Oura,
    Aiken,
    Dolos,
    Zellij,
    Neovim,
    Mithril,
    Scrolls,
    CardanoCli,
    CardanoNode,
    SidechainCli,
    PartnerChainCli,
    PartnerChainNode,
    CardanoSubmitApi,
}

/// Macro to execute a command based on the provided variant and package type.
///
/// This macro matches the provided command against a list of command variants
/// and executes the corresponding code for each variant. It creates a new
/// `Package` instance with the specified package type and client, and then
/// lists the package versions.
///
/// # Parameters
///
/// - `$command`: The command to be matched and executed.
/// - `$client`: The client to be used for creating the `Package`.
/// - `$(($variant:ident, $package_type:expr)),*`: A list of tuples containing
///   the command variant and the corresponding package type.
macro_rules! execute {
    ($command:expr, $client:expr, $(($variant:ident, $package_type:expr)),*) => {
        match $command {
            $(
                Commands::$variant => {
                    let package = Package::new($package_type, String::new(), $client).await;
                    list(package).await.expect("Failed to list versions")
                }
            )*
        }
    }
}

pub async fn run(
    args: Args,
    _ctx: &crate::Context,
    client: Option<&reqwest::Client>,
) -> miette::Result<()> {
    execute!(
        args.command,
        client,
        (Reth, PackageType::Reth),
        (Oura, PackageType::Oura),
        (Aiken, PackageType::Aiken),
        (Dolos, PackageType::Dolos),
        (Zellij, PackageType::Zellij),
        (Neovim, PackageType::Neovim),
        (Mithril, PackageType::Mithril),
        (Scrolls, PackageType::Scrolls),
        (CardanoCli, PackageType::CardanoCli),
        (CardanoNode, PackageType::CardanoNode),
        (SidechainCli, PackageType::SidechainCli),
        (PartnerChainCli, PackageType::PartnerChainCli),
        (PartnerChainNode, PackageType::PartnerChainNode),
        (CardanoSubmitApi, PackageType::CardanoSubmitApi)
    );

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
