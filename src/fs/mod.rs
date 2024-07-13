use std::{fs, path::PathBuf};

use anyhow::{anyhow, Result};

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
