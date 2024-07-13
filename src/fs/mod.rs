use std::{fs, path::PathBuf};

use anyhow::{anyhow, Result};
use semver::Version;

/// Returns the home directory path for the current user.
///
/// This function checks the target operating system using the `cfg!` macro and constructs the home directory path accordingly.
/// For Windows, it uses the "USERPROFILE" environment variable.
/// For macOS, it uses the "/Users/" directory and appends the "SUDO_USER" or "USER" environment variable if they exist and correspond to a valid directory.
/// For other operating systems, it uses the "/home/" directory and appends the "SUDO_USER" or "USER" environment variable if they exist and correspond to a valid directory.
/// If none of the above methods work, it uses the "HOME" environment variable.
///
/// # Returns
///
/// This function returns a `Result` that contains a `PathBuf` representing the home directory path if the operation was successful.
/// If the operation failed, the function returns `Err` with a description of the error.
///
/// # Example
///
/// ```rust
/// let home_dir = get_home_dir()?;
/// ```
pub fn get_home_dir() -> Result<PathBuf> {
  let mut home_str = PathBuf::new();

  if cfg!(windows) {
    home_str.push(std::env::var("USERPROFILE")?);
    return Ok(home_str);
  }

  if cfg!(target_os = "macos") {
    home_str.push("/Users/");
  } else {
    home_str.push("/home/")
  };

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
/// This function first gets the home directory path by calling the `get_home_dir` function.
/// It then checks the target operating system using the `cfg!` macro and constructs the local data directory path accordingly.
/// For Windows, it appends "AppData/Local" to the home directory path.
/// For other operating systems, it appends ".local/share" to the home directory path.
///
/// # Returns
///
/// This function returns a `Result` that contains a `PathBuf` representing the local data directory path if the operation was successful.
/// If the operation failed, the function returns `Err` with a description of the error.
///
/// # Example
///
/// ```rust
/// let local_data_dir = get_local_data_dir()?;
/// ```
pub fn get_local_data_dir() -> Result<PathBuf> {
  let mut home_dir = get_home_dir()?;
  if cfg!(windows) {
    home_dir.push("AppData/Local");
    return Ok(home_dir);
  }

  home_dir.push(".local/share");
  Ok(home_dir)
}

/// Asynchronously returns the downloads directory path based on the application configuration.
///
/// # Returns
///
/// This function returns a `Result` that contains a `PathBuf` representing the downloads directory path if the operation was successful.
/// If the operation failed, the function returns `Err` with a description of the error.
///
/// # Example
///
/// ```rust
/// let downloads_directory = get_downloads_directory().await?;
/// ```
pub async fn get_downloads_directory() -> Result<PathBuf> {
  let mut data_dir = get_local_data_dir()?;

  data_dir.push("hyper-jump");
  let does_folder_exist = tokio::fs::metadata(&data_dir).await.is_ok();
  let is_folder_created = tokio::fs::create_dir_all(&data_dir).await.is_ok();

  if !does_folder_exist && !is_folder_created {
    return Err(anyhow!("Couldn't create downloads directory"));
  }

  Ok(data_dir)
}

/// Returns the file type binary download based on the target operating system.
///
/// This function checks the target operating system using the `cfg!` macro and returns a string that corresponds to the appropriate file type binary download.
/// For Windows, it returns "zip".
/// For macOS, it returns "tar.gz".
/// For other operating systems, it returns "appimage".
///
/// # Returns
///
/// This function returns a `&'static str` that corresponds to the file type binary download.
///
/// # Example
///
/// ```rust
/// let file_type = get_file_type();
/// ```
pub fn get_file_type() -> &'static str {
  if cfg!(target_family = "windows") {
    "zip"
  } else if cfg!(target_os = "macos") {
    "tar.gz"
  } else {
    "appimage"
  }
}

/// Returns the platform-specific name.
///
/// This function takes an `Option<Version>` as an argument, which represents the version to be downloaded.
/// It checks the target operating system and architecture using the `cfg!` macro and returns a string that corresponds to the appropriate download for the platform.
/// For Windows, it returns "win64".
/// For macOS, it checks the version. If the version is less than or equal to 0.9.5, it returns "macos". If the target architecture is "aarch64", it returns "macos-arm64". Otherwise, it returns "macos-x86_64".
///
/// # Arguments
///
/// * `version` - An `Option<Version>` representing the version to be downloaded.
///
/// # Returns
///
/// This function returns a `&'static str` that corresponds to the platform-specific name for download.
///
/// # Example
///
/// ```rust
/// let version = Some(Version::new(0, 9, 5));
/// let platform_name = get_platform_name_download(&version);
/// ```
pub fn get_platform_name_download(version: &Option<Version>) -> &'static str {
  if cfg!(target_os = "windows") {
    "win64"
  } else if cfg!(target_os = "macos") {
    if version
      .as_ref()
      .map_or(false, |x| x <= &Version::new(0, 9, 5))
    {
      "macos"
    } else if cfg!(target_arch = "aarch64") {
      "macos-arm64"
    } else {
      "macos-x86_64"
    }
  } else {
    "linux"
  }
}
