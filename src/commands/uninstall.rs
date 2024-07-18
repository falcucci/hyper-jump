use anyhow::anyhow;
use anyhow::Error;
use anyhow::Result;
use tokio::fs;
use tracing::info;

use super::install::Package;

pub async fn uninstall(package: Package) -> Result<(), Error> {
    let downloads = crate::fs::get_downloads_directory(package.clone()).await?;
    let installation_dir = crate::fs::get_installation_directory().await?;

    if fs::remove_dir_all(&installation_dir).await.is_ok() {
        info!("Successfully removed hyper-jump installation folder");
    }

    if fs::remove_dir_all(downloads).await.is_ok() {
        // For some weird reason this check doesn't really work for downloads folder
        // as it keeps thinking the folder exists and it runs with no issues even tho
        // the folder doesn't exist damn...
        info!("Successfully removed hyper-jump downloads folder");
    } else {
        return Err(anyhow!("There's nothing to uninstall"));
    }

    Ok(())
}
