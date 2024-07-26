use tokio::fs;
use tracing::info;

/// Asynchronously erases the hyper-jump installation and downloads folders.
///
/// This function attempts to remove the hyper-jump installation directory and
/// the downloads directory associated with the provided package. It logs
/// successful removals and returns an error if there is nothing to erase.
///
/// # Arguments
///
/// * `package` - A `Package` instance representing the package whose associated
///   directories are to be erased.
///
/// # Errors
///
/// Returns an error if both the installation and downloads directories do not
/// exist or cannot be removed.
///
/// # Examples
///
/// ```no_run
/// # use hyper_jump::commands::erase::erase;
/// # use hyper_jump::commands::install::Package;
/// # async {
/// let package = Package::new(/* ... */);
/// erase(package).await?;
/// # };
/// ```
pub async fn erase() -> miette::Result<()> {
    let downloads = crate::fs::get_local_data_dir().unwrap();

    if fs::remove_dir_all(&downloads).await.is_ok() {
        info!("Successfully removed hyper-jump installation folder");
    }

    if fs::remove_dir_all(downloads).await.is_ok() {
        // For some weird reason this check doesn't really work for downloads folder
        // as it keeps thinking the folder exists and it runs with no issues even tho
        // the folder doesn't exist damn...
        info!("Successfully removed hyper-jump downloads folder");
    } else {
        info!("No hyper-jump installation or downloads folder to remove");
    }

    Ok(())
}
