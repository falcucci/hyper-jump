use std::path::Path;
use std::path::PathBuf;

use crate::ports::Fs;

#[derive(Clone, Copy)]
pub struct TokioFs;

impl Fs for TokioFs {
    async fn ensure_dir(&self, path: &Path) -> anyhow::Result<()> {
        tokio::fs::create_dir_all(path).await?;
        Ok(())
    }

    async fn copy(&self, src: &Path, dest: &Path) -> anyhow::Result<()> {
        tokio::fs::copy(src, dest).await?;
        Ok(())
    }

    async fn read_dir(&self, path: &Path) -> anyhow::Result<Vec<PathBuf>> {
        let mut dir = tokio::fs::read_dir(path).await?;
        let mut entries = Vec::new();
        while let Some(entry) = dir.next_entry().await? {
            entries.push(entry.path());
        }
        Ok(entries)
    }

    async fn remove_dir_all(&self, path: &Path) -> anyhow::Result<()> {
        tokio::fs::remove_dir_all(path).await?;
        Ok(())
    }

    async fn remove_file(&self, path: &Path) -> anyhow::Result<()> {
        tokio::fs::remove_file(path).await?;
        Ok(())
    }

    async fn set_current_dir(&self, path: &Path) -> anyhow::Result<()> {
        std::env::set_current_dir(path)?;
        Ok(())
    }

    async fn is_dir(&self, path: &Path) -> anyhow::Result<bool> {
        Ok(tokio::fs::metadata(path).await?.is_dir())
    }

    async fn exists(&self, path: &Path) -> anyhow::Result<bool> {
        match tokio::fs::metadata(path).await {
            Ok(_) => Ok(true),
            Err(err) if err.kind() == std::io::ErrorKind::NotFound => Ok(false),
            Err(err) => Err(err.into()),
        }
    }
}
