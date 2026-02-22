//! Hexagonal ports: abstractions the application core depends on.
//!
//! These traits are intentionally small and async-friendly so that adapters can
//! wrap HTTP, filesystem, or console concerns without leaking implementation
//! details into the domain.

use std::fs::File;
use std::path::Path;
use std::path::PathBuf;

use crate::domain::package::Package;
use crate::domain::package::PackageSpec;
use crate::domain::version::LocalVersion;
use crate::domain::version::ParsedVersion;
use crate::domain::version::RemoteVersion;

pub trait ReleaseProvider: Send + Sync {
    async fn latest(&self, package: &PackageSpec) -> anyhow::Result<ParsedVersion>;
    async fn list(&self, package: &PackageSpec) -> anyhow::Result<Vec<RemoteVersion>>;
}

pub trait Downloader: Send + Sync {
    async fn download(&self, url: &str, dest: &Path) -> anyhow::Result<()>;
}

pub trait Archive: Send + Sync {
    async fn extract(&self, package: Package, file: LocalVersion) -> anyhow::Result<()>;
}

pub trait Fs: Send + Sync {
    async fn ensure_dir(&self, path: &Path) -> anyhow::Result<()>;
    async fn copy(&self, src: &Path, dest: &Path) -> anyhow::Result<()>;
    async fn read_dir(&self, path: &Path) -> anyhow::Result<Vec<PathBuf>>;
    async fn remove_dir_all(&self, path: &Path) -> anyhow::Result<()>;
    async fn remove_file(&self, path: &Path) -> anyhow::Result<()>;
    async fn set_current_dir(&self, path: &Path) -> anyhow::Result<()>;
    async fn is_dir(&self, path: &Path) -> anyhow::Result<bool>;
    async fn exists(&self, path: &Path) -> anyhow::Result<bool>;
}

pub trait Paths: Send + Sync {
    async fn downloads_dir(&self, package: Package) -> anyhow::Result<PathBuf>;
    async fn installation_dir(&self) -> anyhow::Result<PathBuf>;
}

pub trait RootDir: Send + Sync {
    async fn root_dir(&self) -> anyhow::Result<PathBuf>;
}

pub trait Platform: Send + Sync {
    fn os(&self) -> &'static str;
    fn arch(&self) -> &'static str;
}

pub trait Output: Send + Sync {
    fn write_line(&self, line: &str) -> anyhow::Result<()>;
}

#[derive(Debug)]
pub struct LockGuard {
    pub(crate) path: Option<PathBuf>,
    pub(crate) file: Option<File>,
}

pub trait Lock: Send + Sync {
    async fn acquire(&self) -> anyhow::Result<LockGuard>;
}

pub trait Env: Send + Sync {
    fn exe_name(&self) -> String;
    fn args(&self) -> Vec<String>;
    fn root_dir(&self) -> Option<PathBuf>;
    fn packages_file(&self) -> Option<PathBuf>;
    fn home_dir(&self) -> Option<PathBuf>;
    fn current_exe(&self) -> anyhow::Result<PathBuf>;
    fn path_var(&self) -> Option<String>;
}

pub trait UsedVersionStore: Send + Sync {
    async fn current(&self, package: Package) -> anyhow::Result<Option<String>>;
    async fn set_current(&self, package: Package, version: &str) -> anyhow::Result<()>;
}

pub trait Process: Send + Sync {
    async fn run(&self, program: &Path, args: &[String]) -> anyhow::Result<()>;
    async fn output(&self, program: &Path, args: &[String]) -> anyhow::Result<Vec<u8>>;
}

pub trait ProxyInstaller: Send + Sync {
    async fn ensure_proxy(&self, alias: &str) -> anyhow::Result<()>;
}
