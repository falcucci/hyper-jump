use crate::adapters::fs::TokioFs;
use crate::adapters::github_release::GitHubReleaseProvider;
use crate::adapters::output::StdoutOutput;
use crate::app::list_remote::list_remote as app_list_remote;
use crate::domain::package::Package;
use crate::domain::package::PackageType;

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
    Jujutsu,
    Mithril,
    Scrolls,
    CardanoCli,
    CardanoNode,
    SidechainCli,
    PartnerChainNode,
    CardanoSubmitApi,
}

/// Macro to execute a command based on the provided variant and package type.
///
/// This macro matches the provided command against a list of command variants
/// and executes the corresponding code for each variant. It creates a new
/// `Package` instance with the specified package type and client, and then
/// lists the remote package versions.
///
/// # Parameters
///
/// - `$command`: The command to be matched and executed.
/// - `$client`: The client to be used for creating the `Package`.
/// - `$(($variant:ident, $package_type:expr)),*`: A list of tuples containing
///   the command variant and the corresponding package type.
macro_rules! execute {
    ($command:expr, $client:expr, $fmt:expr, $paths:expr, $used:expr, $fs:expr, $platform:expr, $(($variant:ident, $package_type:expr)),*) => {
        match $command {
            $(
                Commands::$variant => {
                    let package_type = $package_type;
                    let binary_path = crate::app::layout::binary_path(package_type.clone(), $platform);
                    let package = Package::from_type(package_type, binary_path);
                    let provider = GitHubReleaseProvider::new($client);
                    let output = StdoutOutput;
                    app_list_remote(
                        package,
                        $fmt.clone(),
                        &provider,
                        &output,
                        &$paths,
                        &$used,
                        &$fs,
                    )
                    .await
                    .expect("Failed to list-remote versions")
                }
            )*
        }
    }
}

pub async fn run(
    args: Args,
    ctx: &crate::Context,
    client: Option<&reqwest::Client>,
) -> miette::Result<()> {
    let paths = crate::adapters::path::FsPaths::new(ctx.dirs.root_dir.clone());
    let used_store = crate::adapters::used_store::UsedFileStore::new(paths.clone());
    let fs = TokioFs;
    let platform = crate::adapters::platform::StdPlatform;
    execute!(
        args.command,
        client,
        ctx.output_format.clone(),
        paths,
        used_store,
        fs,
        &platform,
        (Reth, PackageType::Reth),
        (Oura, PackageType::Oura),
        (Aiken, PackageType::Aiken),
        (Dolos, PackageType::Dolos),
        (Zellij, PackageType::Zellij),
        (Neovim, PackageType::Neovim),
        (Jujutsu, PackageType::Jujutsu),
        (Mithril, PackageType::Mithril),
        (Scrolls, PackageType::Scrolls),
        (CardanoCli, PackageType::CardanoCli),
        (CardanoNode, PackageType::CardanoNode),
        (SidechainCli, PackageType::SidechainCli),
        (PartnerChainNode, PackageType::PartnerChainNode),
        (CardanoSubmitApi, PackageType::CardanoSubmitApi)
    );

    Ok(())
}
