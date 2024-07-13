use std::borrow::Cow;
use std::cmp::min;
use std::env;
use std::path::Path;

use anyhow::Error;
use anyhow::{anyhow, Result};
use futures_util::stream::StreamExt;
use indicatif::{ProgressBar, ProgressStyle};
use reqwest::Client;

use tokio::fs::File;
use tokio::io::AsyncWriteExt;

use crate::fs::{get_file_type, get_platform_name_download};
use crate::helpers::version::LocalVersion;
use crate::{
  fs::get_downloads_directory,
  helpers::version::{is_version_installed, ParsedVersion, VersionType},
};

use super::PostDownloadVersionType;

#[derive(Debug)]
pub struct CardanoNode {
  pub url: String,
  pub alias: String,
  pub version: String,
}

#[derive(Debug)]
pub enum Package {
  CardanoNode(CardanoNode),
  CardanoCli,
  Mithril,
}

impl Package {
  pub fn url(&self) -> Option<Cow<str>> {
    match self {
      Package::CardanoNode(CardanoNode { url, version, .. }) => {
        let package_url = url.replace("{version}", version);

        Some(Cow::Owned(format!("https://github.com/{}", package_url)))
      }
      _ => None,
    }
  }
}

pub async fn install(
  client: &Client,
  package: Package,
  version: ParsedVersion,
) -> Result<(), Error> {
  println!("installing package: {:?}", package);
  let root = get_downloads_directory().await?;

  env::set_current_dir(&root)?;
  let root = root.as_path();

  println!("version: {:?}", version);

  let is_version_installed = is_version_installed(&version.tag_name).await?;
  println!("is_version_installed: {:?}", is_version_installed);

  let downloaded_file = match version.version_type {
    VersionType::Normal | VersionType::Latest => {
      download_version(client, &version, root, package).await?
    }
    VersionType::Hash => todo!(),
    // VersionType::Hash => handle_building_from_source(version).await,
    // VersionType::Latest => download_latest_version(client, version, root).await,
  };

  // let package = package::Package::new(package, version)?;
  // let package = package.resolve(client)?;
  //
  // let package_dir = dirs::package_dir(&package.name, &package.version);
  // let package_dir = package_dir.as_path();
  //
  // if package_dir.exists() {
  //   return Err(Error::PackageAlreadyInstalled {
  //     package: package.name.clone(),
  //     version: package.version.clone(),
  //   });
  // }
  //
  // let package_tarball = dirs::package_tarball(&package.name, &package.version);
  // let package_tarball = package_tarball.as_path();
  //
  // let package_tarball_url = package.tarball_url();
  // let package_tarball_url = package_tarball_url.as_str();
  //
  // let package_tarball_response = client.get(package_tarball_url).send()?;
  // let package_tarball_response = package_tarball_response.error_for_status()?;
  //
  // let package_tarball_bytes = package_tarball_response.bytes()?;
  // let package_tarball_bytes = package_tarball_bytes.as_ref();
  //
  // fs::create_dir_all(package_dir)?;
  // fs::write(package_tarball, package_tarball_bytes)?;
  //
  // let package_tarball = fs::File::open(package_tarball)?;
  // let package_tarball = flate2::read::GzDecoder::new(package_tarball);
  // let package_tarball = tar::Archive::new(package_tarball);
  //
  // package_tarball.unpack(package_dir)?;

  Ok(())
}

/// This function sends a request to download the specified version based on the version type.
/// If the version type is Normal, Nightly, or Latest, it sends a request to download the version.
/// If the version type is Hash, it handles building from source.
/// If the version type is NightlyRollback, it does nothing.
///
/// # Arguments
///
/// * `client` - A reference to the HTTP client.
/// * `version` - A reference to the parsed version to be downloaded.
/// * `root` - A reference to the path where the downloaded file will be saved.
///
/// # Returns
///
/// * `Result<PostDownloadVersionType>` - Returns a `Result` that contains a `PostDownloadVersionType` on success, or an error on failure.
///
/// # Errors
///
/// This function will return an error if:
/// * There is a failure in sending the request to download the version.
/// * The response status is not 200.
/// * There is a failure in creating the file where the downloaded version will be saved.
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
  client: &Client,
  version: &ParsedVersion,
  root: &Path,
  package: Package,
) -> Result<PostDownloadVersionType> {
  println!("package: {:?}", package);
  match version.version_type {
    VersionType::Normal | VersionType::Latest => {
      let response = send_request(client, version, package).await;

      match response {
        Ok(response) => {
          if response.status() == 200 {
            let total_size = response.content_length().unwrap();
            let mut response_bytes = response.bytes_stream();

            // Progress Bar Setup
            let pb = ProgressBar::new(total_size);
            pb.set_style(ProgressStyle::default_bar()
                    .template("{msg}\n{spinner:.green} [{elapsed_precise}] [{wide_bar:.cyan/blue}] {bytes}/{total_bytes} ({bytes_per_sec}, {eta})")
                    .unwrap()
                    .progress_chars("█  "));
            pb.set_message(format!("Downloading version: {}", version.tag_name));

            let file_type = get_file_type();
            let mut file =
              tokio::fs::File::create(format!("{}.{file_type}", version.tag_name)).await?;

            let mut downloaded: u64 = 0;

            while let Some(item) = response_bytes.next().await {
              let chunk = item.map_err(|_| anyhow!("hello"))?;
              file.write_all(&chunk).await?;
              let new = min(downloaded + (chunk.len() as u64), total_size);
              downloaded = new;
              pb.set_position(new);
            }

            pb.finish_with_message(format!(
              "Downloaded version {} to {}/{}.{file_type}",
              version.tag_name,
              root.display(),
              version.tag_name
            ));

            Ok(PostDownloadVersionType::Standard(LocalVersion {
              file_name: version.tag_name.to_owned(),
              file_format: file_type.to_string(),
              path: root.display().to_string(),
              semver: version.semver.clone(),
            }))
          } else {
            Err(anyhow!(
              "Please provide an existing neovim version, {}",
              response.text().await?
            ))
          }
        }
        Err(error) => Err(anyhow!(error)),
      }
    }
    // VersionType::Hash => handle_building_from_source(version).await,
    VersionType::Hash => todo!(),
    VersionType::Latest => todo!(),
  }
}

/// Sends a GET request to the specified URL to download a specific version.
///
/// # Arguments
///
/// * `client: &Client` - A reference to the `Client` used for making requests.
/// * `version: &ParsedVersion` - Contains the version information to be downloaded.
///
/// It then sends a GET request to the constructed URL with the header "user-agent" set to
/// "hyper-jump".
///
/// # Returns
///
/// * `Result<reqwest::Response, reqwest::Error>` - Returns a `Result` containing the server's
/// response to the GET request. If the request fails, it returns an error.
///
/// # Example
///
/// ```rust
/// let client = Client::new();
/// let version = ParsedVersion { tag_name: "v8.1.2", semver: Version::parse("8.1.2").unwrap() };
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
  client: &Client,
  version: &ParsedVersion,
  package: Package,
) -> Result<reqwest::Response, reqwest::Error> {
  let platform = get_platform_name_download(&version.semver);
  println!("platform: {:?}", platform);
  let file_type = get_file_type();

  let package_url = package.url().unwrap();
  println!("package_url: {:?}", package_url);

  client
    .get(package_url.to_string())
    .header("user-agent", "hyper-jump")
    .send()
    .await
}
