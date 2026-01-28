use std::path::PathBuf;

use crate::domain::package::Package;
use crate::ports::Paths;

#[derive(Clone)]
pub struct FsPaths {
    root_dir: PathBuf,
}

impl FsPaths {
    pub fn new(root_dir: PathBuf) -> Self { Self { root_dir } }
}

impl Paths for FsPaths {
    async fn downloads_dir(&self, package: Package) -> anyhow::Result<PathBuf> {
        let mut root = self.root_dir.clone();
        let alias = package.alias();
        root.push(alias);
        tokio::fs::create_dir_all(&root).await?;
        Ok(root)
    }

    async fn installation_dir(&self) -> anyhow::Result<PathBuf> {
        let mut root = self.root_dir.clone();
        root.push("bin");
        tokio::fs::create_dir_all(&root).await?;
        Ok(root)
    }
}
