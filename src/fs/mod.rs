use std::env;
use std::fs;
use std::path::Path;
use std::path::PathBuf;
use std::process::Command;

use anyhow::anyhow;
use anyhow::Result;
use tracing::info;

use crate::helpers::version::LocalVersion;
use crate::packages::Package;
use crate::packages::PackageType;

/// Returns the home directory path for the current user.
///
/// This function checks the target operating system using the `cfg!` macro and
/// constructs the home directory path accordingly. For Windows, it uses the
/// "USERPROFILE" environment variable. For macOS, it uses the "/Users/"
/// directory and appends the "SUDO_USER" or "USER" environment variable if they
/// exist and correspond to a valid directory. For other operating systems, it
/// uses the "/home/" directory and appends the "SUDO_USER" or "USER"
/// environment variable if they exist and correspond to a valid directory.
/// If none of the above methods work, it uses the "HOME" environment variable.
///
/// # Returns
///
/// This function returns a `Result` that contains a `PathBuf` representing the
/// home directory path if the operation was successful. If the operation
/// failed, the function returns `Err` with a description of the error.
///
/// # Example
///
/// ```rust
/// let home_dir = get_home_dir()?; 
/// ```
pub fn get_home_dir() -> Result<PathBuf> {
    let mut home_str = PathBuf::new();

    #[cfg(target_family = "windows")]
    {
        home_str.push(std::env::var("USERPROFILE")?);
        return Ok(home_str);
    }

    #[cfg(target_os = "macos")]
    {
        home_str.push("/Users/");
    }

    #[cfg(target_os = "linux")]
    {
        home_str.push("/home/");
    }

    if let Ok(value) = std::env::var("SUDO_USER") {
        home_str.push(&value);
        if fs::metadata(&home_str).is_ok() {
            return Ok(home_str);
        }
    }

    if let Ok(value) = std::env::var("USER") {
        home_str.push(&value);
        if fs::metadata(&home_str).is_ok() {
            return Ok(home_str);
        }
    }

    let home_value = std::env::var("HOME")?;
    home_str = PathBuf::from(home_value);

    Ok(home_str)
}

/// Returns the local data directory path for the current user.
///
/// This function first gets the home directory path by calling the
/// `get_home_dir` function. It then checks the target operating system using
/// the `cfg!` macro and constructs the local data directory path accordingly.
/// For Windows, it appends "AppData/Local" to the home directory path.
/// For other operating systems, it appends ".local/share" to the home directory
/// path.
///
/// # Returns
///
/// This function returns a `Result` that contains a `PathBuf` representing the
/// local data directory path if the operation was successful. If the operation
/// failed, the function returns `Err` with a description of the error.
///
/// # Example
///
/// ```rust
/// let local_data_dir = get_local_data_dir()?; 
/// ```
pub fn get_local_data_dir() -> Result<PathBuf> {
    let mut home_dir = get_home_dir()?;

    #[cfg(target_family = "windows")]
    home_dir.push("AppData/Local");

    home_dir.push(".local/share");
    home_dir.push("hyper-jump");

    Ok(home_dir)
}

/// Asynchronously returns the downloads directory path based on the application
/// configuration.
///
/// # Returns
///
/// This function returns a `Result` that contains a `PathBuf` representing the
/// downloads directory path if the operation was successful. If the operation
/// failed, the function returns `Err` with a description of the error.
///
/// # Example
///
/// ```rust
/// let downloads_directory = get_downloads_directory().await?; 
/// ```
pub async fn get_downloads_directory(package: Package) -> Result<PathBuf> {
    let mut data_dir = get_local_data_dir()?;
    let alias = package.alias();
    data_dir.push(alias);

    let does_folder_exist = tokio::fs::metadata(&data_dir).await.is_ok();
    let is_folder_created = tokio::fs::create_dir_all(&data_dir).await.is_ok();

    if !does_folder_exist && !is_folder_created {
        return Err(anyhow!("Couldn't create downloads directory"));
    }

    Ok(data_dir)
}

/// Returns the file type binary download based on the target operating system.
///
/// This function checks the target operating system using the `cfg!` macro and
/// returns a string that corresponds to the appropriate file type binary
/// download. For Windows, it returns "zip".
/// For macOS, it returns "tar.gz".
/// For other operating systems, it returns "appimage".
///
/// # Returns
///
/// This function returns a `&'static str` that corresponds to the file type
/// binary download.
///
/// # Example
///
/// ```rust
/// let file_type = get_file_type(); 
/// ```
pub fn get_file_type() -> &'static str {
    #[cfg(target_family = "windows")]
    {
        "zip"
    }

    #[cfg(target_os = "macos")]
    {
        "tar.gz"
    }

    #[cfg(target_os = "linux")]
    {
        "tar.gz"
    }
}

/// Returns the platform-specific name.
///
/// This function takes an `Option<Version>` as an argument, which represents
/// the version to be downloaded. It checks the target operating system and
/// architecture using the `cfg!` macro and returns a string that corresponds to
/// the appropriate download for the platform. For Windows, it returns "win64".
/// For macOS, it checks the version. If the version is less than or equal to
/// 0.9.5, it returns "macos". If the target architecture is "aarch64", it
/// returns "macos-arm64". Otherwise, it returns "macos-x86_64".
///
/// # Arguments
///
/// * `version` - An `Option<Version>` representing the version to be
///   downloaded.
///
/// # Returns
///
/// This function returns a `&'static str` that corresponds to the
/// platform-specific name for download.
///
/// # Example
///
/// ```rust
/// let platform_name = get_platform_name_download(); 
/// ```
pub fn get_platform_name() -> &'static str { std::env::consts::OS }

/// Retrieves the platform-specific name for downloads based on the target
/// operating system.
///
/// # Examples
///
/// ```
/// let platform_name_download = get_platform_name_download();
/// println!("Platform name for downloads: {}", platform_name_download);
/// ```
pub fn get_platform_name_download(package_type: PackageType) -> &'static str {
    #[cfg(target_family = "windows")]
    {
        "win64"
    }

    #[cfg(target_os = "macos")]
    {
        #[cfg(target_arch = "aarch64")]
        {
            match package_type {
                PackageType::CardanoNode => "",
                PackageType::CardanoCli => "",
                PackageType::Mithril => "arm64",
                PackageType::Aiken => "aarch64-apple-darwin",
            }
        }

        #[cfg(target_arch = "x86_64")]
        {
            match package_type {
                PackageType::CardanoNode => "",
                PackageType::CardanoCli => "",
                PackageType::Mithril => "x86_64",
                PackageType::Aiken => "x86_64-apple-darwin",
            }
        }
    }

    #[cfg(target_os = "linux")]
    {
        match package_type {
            PackageType::CardanoNode => "",
            PackageType::CardanoCli => "",
            PackageType::Mithril => "x64",
            PackageType::Aiken => "x86_64-unknown-linux-gnu",
        }
    }
}

/// Copies the proxy to the installation directory.
///
/// This function gets the current executable's path, determines the
/// installation directory, creates it if it doesn't exist, adds it to the
/// system's PATH, and copies the current executable to the installation
/// directory as "cardano-node".
///
/// # Returns
///
/// * `Result<()>` - Returns a `Result` that indicates whether the operation was
///   successful or not.
///
/// # Errors
///
/// This function will return an error if:
///
/// * The current executable's path cannot be determined.
/// * The installation directory cannot be created.
/// * The installation directory cannot be added to the PATH.
/// * The version of the existing file cannot be determined.
/// * The existing file cannot be replaced.
///
/// # Example
///
/// ```rust
/// let package = Package::CardanoNode(CardanoNode {
///     alias: "cardano-node".to_string(),
///     version: "1.0.0".to_string(),
///     url: "https://example.com".to_string(),
/// });
///
/// copy_package_proxy(package).await.unwrap();
/// ```
pub async fn copy_package_proxy(package: Package) -> Result<()> {
    let exe_path = env::current_exe().unwrap();
    let mut installation_dir = get_installation_directory().await?;

    if fs::metadata(&installation_dir).is_err() {
        fs::create_dir_all(&installation_dir)?;
    }

    add_to_path(&installation_dir)?;

    let alias = package.alias();
    installation_dir.push(&alias);
    if fs::metadata(&installation_dir).is_ok() {
        let output = Command::new(&alias).arg("--&hyper-jump").output()?.stdout;
        let version = String::from_utf8(output)?.trim().to_string();

        if version == env!("CARGO_PKG_VERSION") {
            return Ok(());
        }
    }

    fs::copy(&exe_path, &installation_dir).map_err(|_| anyhow!("Could not copy the proxy"))?;

    Ok(())
}

/// Adds the installation directory to the system's PATH.
///
/// This function checks if the installation directory is already in the PATH.
/// If not, it adds the directory to the PATH.
///
/// # Arguments
///
/// * `installation_dir` - The directory to be added to the PATH.
///
/// # Returns
///
/// * `Result<()>` - Returns a `Result` that indicates whether the operation was
///   successful or not.
///
/// # Errors
///
/// This function will return an error if:
///
/// * The installation directory cannot be converted to a string.
/// * The current user's environment variables cannot be accessed or modified
///   (Windows only).
/// * The PATH environment variable cannot be read (non-Windows only).
///
/// # Example
///
/// ```rust
/// let installation_dir = Path::new("/usr/local/bin");
/// add_to_path(&installation_dir).unwrap();
/// ```
fn add_to_path(installation_dir: &Path) -> Result<()> {
    let installation_dir = installation_dir.to_str().unwrap();

    if !std::env::var("PATH")?.contains("cardano-bin") {
        info!("Make sure to have {installation_dir} in PATH");
    }

    Ok(())
}

/// Asynchronously returns the installation directory path based on the
/// application configuration.
///
/// If the `installation_location` field in the `Config` is not set, it gets the
/// downloads directory path by calling the `get_downloads_directory` function
/// and appends "cardano-node-bin" to it.
///
/// # Returns
///
/// This function returns a `Result` that contains a `PathBuf` representing the
/// installation directory path if the operation was successful.
/// If the operation failed, the function returns `Err` with a description of
/// the error.
///
/// # Example
///
/// ```rust
/// let installation_directory = get_installation_directory().await?; 
/// ```
pub async fn get_installation_directory() -> Result<PathBuf> {
    let mut installation_location = get_local_data_dir()?;

    installation_location.push("cardano-bin");

    Ok(installation_location)
}

/// Starts the process of expanding a downloaded file.
///
/// This function is asynchronous and uses `tokio::task::spawn_blocking` to run
/// the `expand` function in a separate thread. It takes a `LocalVersion` struct
/// which contains information about the downloaded file, such as its name,
/// format, and path. The function first clones the `LocalVersion` struct and
/// passes it to the `expand` function. If the `expand` function returns an
/// error, the `start` function also returns an error. If the `expand` function
/// is successful, the `start` function removes the original downloaded file.
///
/// # Arguments
///
/// * `file` - A `LocalVersion` struct representing the downloaded file.
///
/// # Returns
///
/// This function returns a `Result` that indicates whether the operation was
/// successful. If the operation was successful, the function returns `Ok(())`.
/// If the operation failed, the function returns `Err` with a description of
/// the error.
///
/// # Errors
///
/// This function will return an error if:
///
/// * The `expand` function returns an error.
/// * The original downloaded file could not be removed.
///
/// # Example
///
/// ```rust
/// let downloaded_file = LocalVersion {
///     file_name: "cardano-node-darwin",
///     file_format: "tar.gz",
///     semver: semver::Version::parse("8.1.2").unwrap(),
///     path: "/path/to/downloaded/file",
/// };
/// unarchive(downloaded_file).await;
/// ```
pub async fn unarchive(package: Package, file: LocalVersion) -> Result<()> {
    let path = format!("{}/{}.{}", file.path, file.file_name, file.file_format);
    tokio::task::spawn_blocking(move || expand(package, file))
        .await?
        .map_err(|e| anyhow!(e))?;

    tokio::fs::remove_file(path).await?;

    Ok(())
}

/// Expands a downloaded file on macOS.
///
/// This function is specific to macOS due to the use of certain features like
/// `os::unix::fs::PermissionsExt`. It takes a `LocalVersion` struct which
/// contains information about the downloaded file, such as its name and format.
/// The function then opens the file, decompresses it using `GzDecoder`, and
/// extracts its contents using `tar::Archive`. During the extraction process, a
/// progress bar is displayed to the user. After extraction, the function
/// renames the `cardano-node-osx64` directory to `cardano-node-macos` if it
/// exists. Finally, it sets the permissions of the `cardano-node` binary to
/// `0o551`.
///
/// # Arguments
///
/// * `downloaded_file` - A `LocalVersion` struct representing the downloaded
///   file.
///
/// # Returns
///
/// This function returns a `Result` that indicates whether the operation was
/// successful. If the operation was successful, the function returns `Ok(())`.
/// If the operation failed, the function returns `Err` with a description of
/// the error.
///
/// # Errors
///
/// This function will return an error if:
///
/// * The downloaded file could not be opened.
/// * The file could not be decompressed or extracted.
/// * The `cardano-node-osx64` directory could not be renamed.
/// * The permissions of the `cardano-node` binary could not be set.
///
/// # Example
///
/// ```rust
/// let downloaded_file = LocalVersion {
///     file_name: "cardano-node-macos",
///     file_format: "tar.gz",
///     semver: semver::Version::parse("0.5.0").unwrap(),
///     path: "/path/to/downloaded/file",
/// };
/// expand(downloaded_file);
/// ```
fn expand(package: Package, tmp: LocalVersion) -> Result<()> {
    use std::fs::File;
    use std::os::unix::fs::PermissionsExt;

    use anyhow::Context;
    use flate2::read::GzDecoder;
    use indicatif::ProgressBar;
    use indicatif::ProgressStyle;
    use tar::Archive;

    if fs::metadata(&tmp.file_name).is_ok() {
        fs::remove_dir_all(&tmp.file_name)?;
    }

    let file_path = format!("{}/{}.{}", tmp.path, tmp.file_name, tmp.file_format);
    let file = File::open(&file_path).map_err(|error| {
        anyhow!(
            "Failed to open file {}.{}, file doesn't exist. additional info: {error}",
            tmp.file_name,
            tmp.file_format,
        )
    })?;

    let output = format!("{}/{}", tmp.path, tmp.file_name);
    let decompress_stream = GzDecoder::new(file);
    Archive::new(decompress_stream).unpack(&output).with_context(|| {
        format!(
            "Failed to decompress or extract file {}.{}",
            tmp.file_name, tmp.file_format
        )
    })?;

    // hard coding this is pretty unwise, but you cant get the length of an
    // archive in tar-rs unlike zip-rs
    let totalsize = 4692;
    let pb = ProgressBar::new(totalsize);
    let pb_style = ProgressStyle::default_bar()
        .template("{msg}\n{spinner:.green} [{elapsed_precise}] [{wide_bar:.cyan/blue}] {pos}/{len}")
        .unwrap()
        .progress_chars("=> ");
    pb.set_style(pb_style);

    pb.finish_with_message(format!(
        "Finished expanding to {}/{}",
        tmp.path, tmp.file_name
    ));

    let binary = &format!(
        "{}/{}/{}",
        tmp.file_name,
        package.binary_path(),
        package.binary_name()
    );
    let mut perms = fs::metadata(binary)?.permissions();
    perms.set_mode(0o551);
    fs::set_permissions(binary, perms)?;

    Ok(())
}
