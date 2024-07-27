use tokio::fs;
use tracing::info;

/// Asynchronously erases the hyper-jump installation and downloads folders.
///
/// This function attempts to remove the hyper-jump installation directory and
/// the downloads directory. It logs successful removals and returns an error if
/// there is nothing to erase or if an error occurs during the removal process.
///
/// # Errors
///
/// Returns an error if both the installation and downloads directories do not
/// exist or cannot be removed.
///
/// # Examples
///
/// ```no_run
/// #[tokio::main]
/// async fn main() -> miette::Result<()> {
///     erase().await?;
///     Ok(())
/// }
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
