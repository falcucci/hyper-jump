use std::path::PathBuf;

use crate::domain::package::Package;
use crate::ports::Paths;
use crate::ports::UsedVersionStore;

pub struct UsedFileStore<P> {
    paths: P,
}

impl<P> UsedFileStore<P> {
    pub fn new(paths: P) -> Self { Self { paths } }
}

impl<P> UsedVersionStore for UsedFileStore<P>
where
    P: Paths,
{
    async fn current(&self, package: Package) -> anyhow::Result<Option<String>> {
        let mut path: PathBuf = self.paths.downloads_dir(package).await?;
        path.push("used");
        match tokio::fs::read_to_string(&path).await {
            Ok(contents) => Ok(Some(contents.trim().to_string())),
            Err(err) if err.kind() == std::io::ErrorKind::NotFound => Ok(None),
            Err(err) => Err(err.into()),
        }
    }

    async fn set_current(&self, package: Package, version: &str) -> anyhow::Result<()> {
        let mut path: PathBuf = self.paths.downloads_dir(package).await?;
        path.push("used");
        tokio::fs::write(path, version).await?;
        Ok(())
    }
}
