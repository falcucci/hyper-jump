use std::cmp::min;
use std::env;
use std::path::Path;

use anyhow::anyhow;
use anyhow::Error;
use anyhow::Result;
use clap::Parser;
use futures_util::stream::StreamExt;
use indicatif::ProgressBar;
use reqwest::Client;
use tokio::io::AsyncWriteExt;
use tracing::info;

use super::PostDownloadVersionType;
use crate::fs::copy_package_proxy;
use crate::fs::get_downloads_directory;
use crate::fs::get_file_type;
use crate::fs::get_platform_name;
use crate::fs::unarchive;
use crate::helpers::version::is_version_installed;
use crate::helpers::version::LocalVersion;
use crate::helpers::version::ParsedVersion;
use crate::helpers::version::VersionType;
use crate::packages::Package;
use crate::packages::PackageType;

#[derive(Parser)]
pub struct Args {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Parser)]
pub enum Commands {
    Oura { version: String },
    Aiken { version: String },
    Dolos { version: String },
    Mithril { version: String },
    Scrolls { version: String },
    CardanoCli { version: String },
    CardanoNode { version: String },
    CardanoDbSync { version: String },
    CardanoSubmitApi { version: String },
}

/// Macro to execute a command based on the provided variant and package type.
///
/// This macro matches the provided command against a list of command variants
/// and executes the corresponding code for each variant. It creates a new
/// `Package` instance with the specified package type and client, and then
/// installs the package.
///
/// # Parameters
///
/// - `$command`: The command to be matched and executed. The command must
///   include a `version`.
/// - `$client`: The client to be used for creating the `Package`.
/// - `$(($variant:ident, $package_type:expr)),*`: A list of tuples containing
///   the command variant and the corresponding package type.
macro_rules! execute {
    ($command:expr, $client:expr, $(($variant:ident, $package_type:expr)),*) => {
        match $command {
            $(
                Commands::$variant { version } => {
                    let package = Package::new($package_type, version, $client).await;
                    install($client, package).await.expect("Failed to install");
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
        (Oura, PackageType::Oura),
        (Aiken, PackageType::Aiken),
        (Dolos, PackageType::Dolos),
        (Mithril, PackageType::Mithril),
        (Scrolls, PackageType::Scrolls),
        (CardanoCli, PackageType::CardanoCli),
        (CardanoNode, PackageType::CardanoNode),
        (CardanoDbSync, PackageType::CardanoDbSync),
        (CardanoSubmitApi, PackageType::CardanoSubmitApi)
    );

    Ok(())
}

/// Installs a specified version of a package asynchronously.
///
/// This function handles the installation process of a package by first
/// checking if the version is already installed, and if not, it proceeds to
/// download and unarchive the specified version.
///
/// # Arguments
///
/// * `client` - A reference to a `reqwest::Client` for making HTTP requests.
/// * `package` - The `Package` to be installed.
/// * `version` - The `ParsedVersion` specifying the version to install.
///
/// # Errors
///
/// Returns an error if the installation process fails at any step, including
/// checking for an already installed version, downloading, or unarchiving the
/// package.
///
/// # Examples
///
/// ```rust
/// let client = Client::new();
/// let package = Package::new_cardano_node("1.0.0".to_string());
/// let version = ParsedVersion::parse("1.0.0").unwrap();
/// install(&client, package, version).await?;
/// ```
pub async fn install(client: Option<&Client>, package: Package) -> Result<(), Error> {
    let version = package.version().map_or_else(|| Err(anyhow!("No version specified")), Ok)?;
    let root = get_downloads_directory(package.clone()).await?;

    env::set_current_dir(&root)?;
    let root = root.as_path();

    let is_version_installed = is_version_installed(&version.tag_name, package.clone()).await?;

    copy_package_proxy(package.clone()).await?;

    if is_version_installed {
        info!("{} is already installed.", version.tag_name);
        return Ok(());
    }

    let downloaded_file = match version.version_type {
        VersionType::Normal | VersionType::Latest => {
            download_version(client, &version, root, package.clone()).await?
        }
    };

    match downloaded_file {
        PostDownloadVersionType::Standard(local_version) => {
            unarchive(package, local_version).await?;
        }
    }

    info!("Successfully installed {}", version.tag_name);

    Ok(())
}

/// This function sends a request to download the specified version based on the
/// version type. If the version type is Normal, Nightly, or Latest, it sends a
/// request to download the version. If the version type is Hash, it handles
/// building from source. If the version type is NightlyRollback, it does
/// nothing.
///
/// # Arguments
///
/// * `client` - A reference to the HTTP client.
/// * `version` - A reference to the parsed version to be downloaded.
/// * `root` - A reference to the path where the downloaded file will be saved.
///
/// # Returns
///
/// * `Result<PostDownloadVersionType>` - Returns a `Result` that contains a
///   `PostDownloadVersionType` on success, or an error on failure.
///
/// # Errors
///
/// This function will return an error if:
/// * There is a failure in sending the request to download the version.
/// * The response status is not 200.
/// * There is a failure in creating the file where the downloaded version will
///   be saved.
/// * There is a failure in writing the downloaded bytes to the file.
///
/// # Example
///
/// ```rust
/// let client = Client::new();
/// let version = ParsedVersion::parse("0.5.0");
/// let root = Path::new("/path/to/save");
/// let result = download_version(&client, &version, &root).await;
/// ```
async fn download_version(
    client: Option<&Client>,
    version: &ParsedVersion,
    root: &Path,
    package: Package,
) -> Result<PostDownloadVersionType> {
    let response = send_request(client, package.clone()).await?;
    if response.status() != reqwest::StatusCode::OK {
        return Err(anyhow!("Failed to send request to download version"));
    }

    let mut downloaded: u64 = 0;
    let content_length = get_content_length(&response).await?;
    let pb = ProgressBar::new(content_length);
    let mut response_bytes = response.bytes_stream();
    let package_type = package.package_type();
    let file_type = get_file_type(package_type);
    let file_path = create_file_path(version, root, file_type);
    let mut file = create_file(&file_path).await?;
    while let Some(item) = response_bytes.next().await {
        let chunk = item.map_err(|_| anyhow!("Failed to get chunk"))?;
        file.write_all(&chunk).await?;
        let new = min(downloaded + (chunk.len() as u64), content_length);
        downloaded = new;
        pb.set_position(new);
    }

    pb.finish_with_message(format!(
        "Downloaded version {} to {}",
        version.tag_name, file_path
    ));

    let local_version = LocalVersion {
        file_name: version.tag_name.to_owned(),
        file_format: file_type.to_string(),
        path: root.display().to_string(),
        semver: version.semver.clone(),
    };

    Ok(PostDownloadVersionType::Standard(local_version))
}

/// Retrieves the content length from an HTTP response.
///
/// This function extracts the `Content-Length` header from the given HTTP
/// response and returns it as a `u64`. If the header is not present, it returns
/// an error.
///
/// # Arguments
///
/// * `response` - A reference to the `reqwest::Response` object.
///
/// # Returns
///
/// This function returns a `Result` indicating the success or failure of the
/// operation.
///
/// * `Ok(u64)` - The content length of the response.
/// * `Err(anyhow::Error)` - An error occurred if the `Content-Length` header is
///   missing.
///
/// # Examples
///
/// ```rust
/// let response = reqwest::get("https://example.com").await?;
/// let content_length = get_content_length(&response).await?;
/// ```
async fn get_content_length(response: &reqwest::Response) -> Result<u64> {
    let content_length = response.content_length();

    content_length.ok_or(anyhow!("Failed to get content length of the response"))
}

/// Creates a new file asynchronously at the specified path.
///
/// This function creates a new file at the given file path using asynchronous
/// file operations provided by `tokio::fs`.
///
/// # Arguments
///
/// * `file_path` - A string slice that holds the path where the file should be
///   created.
///
/// # Returns
///
/// This function returns a `Result` indicating the success or failure of the
/// file creation.
///
/// * `Ok(tokio::fs::File)` - The created file handle.
/// * `Err(anyhow::Error)` - An error occurred during file creation.
///
/// # Examples
///
/// ```rust
/// let file_path = "/tmp/example.txt";
/// let file = create_file(file_path).await?;
/// ```
async fn create_file(file_path: &str) -> Result<tokio::fs::File> {
    Ok(tokio::fs::File::create(&file_path).await?)
}

/// Constructs a file path string based on the version, root path, and file
/// type.
///
/// This function generates a file path string by combining the root path,
/// version tag name, and file type. The resulting path is formatted as
/// `root/tag_name.file_type`.
///
/// # Arguments
///
/// * `version` - A reference to a `ParsedVersion` object containing the version
///   information.
/// * `root` - A reference to a `Path` object representing the root directory.
/// * `file_type` - A string slice representing the file extension or type.
///
/// # Returns
///
/// This function returns a `String` representing the constructed file path.
///
/// # Examples
///
/// ```rust
/// let version = ParsedVersion {
///     tag_name: "v1.0.0".to_string(),
/// };
/// let root = Path::new("/tmp");
/// let file_type = "txt";
/// let file_path = create_file_path(&version, &root, file_type);
/// ```
fn create_file_path(version: &ParsedVersion, root: &Path, file_type: &str) -> String {
    format!("{}/{}.{}", root.display(), version.tag_name, file_type)
}

/// Sends a GET request to the specified URL to download a specific version.
///
/// # Arguments
///
/// * `client: &Client` - A reference to the `Client` used for making requests.
/// * `version: &ParsedVersion` - Contains the version information to be
///   downloaded.
///
/// It then sends a GET request to the constructed URL with the header
/// "user-agent" set to "hyper-jump".
///
/// # Returns
///
/// * `Result<reqwest::Response, reqwest::Error>` - Returns a `Result`
///   containing the server's
/// * response to the GET request. If the request fails, it returns an error.
///
/// # Example
///
/// ```rust
/// let client = Client::new();
/// let response = send_request(&client, &version).await?;
/// ```
///
/// # Note
///
/// This function is asynchronous and must be awaited.
///
/// # See Also
///
/// * [`helpers::get_platform_name_download`](src/helpers/platform.rs)
/// * [`helpers::get_file_type`](src/helpers/file.rs)
async fn send_request(
    client: Option<&Client>,
    package: Package,
) -> Result<reqwest::Response, reqwest::Error> {
    let platform = get_platform_name();
    let package_type = package.package_type();
    let file_type = get_file_type(package_type);

    let package_url = package.download_url();
    info!("Downloading: {}", package_url);

    client
        .expect("Client is not set")
        .get(package_url.to_string())
        .header("user-agent", "hyper-jump")
        .send()
        .await
}
