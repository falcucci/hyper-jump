use std::path::PathBuf;

use fs4::FileExt;
use tokio::task;

use crate::ports::Lock;
use crate::ports::LockGuard;
use crate::ports::Paths;

pub struct FileLock {
    path: PathBuf,
}

impl FileLock {
    pub async fn from_paths<P: Paths>(paths: &P) -> anyhow::Result<Self> {
        let base = paths.installation_dir().await?;
        Ok(Self {
            path: base.join("hyper-jump.lock"),
        })
    }
}

impl Lock for FileLock {
    async fn acquire(&self) -> anyhow::Result<LockGuard> {
        let path = self.path.clone();
        let file = task::spawn_blocking(move || -> anyhow::Result<std::fs::File> {
            use std::io::Write;
            let mut file =
                std::fs::OpenOptions::new().read(true).write(true).create(true).open(&path)?;
            file.lock_exclusive()?;
            let _ = file.set_len(0);
            let _ = writeln!(file, "pid:{}", std::process::id());
            Ok(file)
        })
        .await??;

        Ok(LockGuard {
            path: Some(self.path.clone()),
            file: Some(file),
        })
    }
}

impl Drop for LockGuard {
    fn drop(&mut self) {
        if let Some(file) = self.file.take() {
            let _ = file.unlock();
        }
        if let Some(path) = self.path.take() {
            let _ = std::fs::remove_file(path);
        }
    }
}
