use tracing::info;

use crate::ports::Fs;
use crate::ports::RootDir;

pub async fn erase(root: &impl RootDir, fs: &impl Fs) -> anyhow::Result<()> {
    let root_dir = root.root_dir().await?;

    if fs.remove_dir_all(&root_dir).await.is_ok() {
        info!("Successfully removed hyper-jump installation folder");
    } else {
        info!("No hyper-jump installation or downloads folder to remove");
    }

    Ok(())
}
