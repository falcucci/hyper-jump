use std::path::Path;

use anyhow::anyhow;
use anyhow::Result;
use tracing::info;

use crate::ports::Env;
use crate::ports::Fs;
use crate::ports::Paths;
use crate::ports::Process;
use crate::ports::ProxyInstaller;

#[derive(Clone)]
pub struct ProxyFsCopier<P, E, F, Proc> {
    paths: P,
    env: E,
    fs: F,
    process: Proc,
}

impl<P, E, F, Proc> ProxyFsCopier<P, E, F, Proc> {
    pub fn new(paths: P, env: E, fs: F, process: Proc) -> Self {
        Self {
            paths,
            env,
            fs,
            process,
        }
    }
}

impl<P, E, F, Proc> ProxyInstaller for ProxyFsCopier<P, E, F, Proc>
where
    P: Paths + Clone + Send + Sync,
    E: Env + Clone + Send + Sync,
    F: Fs + Clone + Send + Sync,
    Proc: Process + Clone + Send + Sync,
{
    async fn ensure_proxy(&self, alias: &str) -> anyhow::Result<()> {
        let paths = self.paths.clone();
        let env = self.env.clone();
        let fs = self.fs.clone();
        let process = self.process.clone();
        copy_package_proxy(&paths, &env, &fs, &process, alias).await
    }
}

async fn copy_package_proxy(
    paths: &impl Paths,
    env: &impl Env,
    fs: &impl Fs,
    process: &impl Process,
    alias: &str,
) -> Result<()> {
    let exe_path = env.current_exe()?;
    let mut installation_dir = paths.installation_dir().await?;

    if !fs.exists(&installation_dir).await? {
        fs.ensure_dir(&installation_dir).await?;
    }

    add_to_path(env, &installation_dir)?;

    installation_dir.push(alias);
    let proxy_version = read_proxy_version(process, alias).await?;
    if matches!(proxy_version, Some(version) if version == env!("CARGO_PKG_VERSION")) {
        return Ok(());
    }

    fs.copy(&exe_path, &installation_dir)
        .await
        .map_err(|_| anyhow!("Could not copy the proxy"))?;

    Ok(())
}

fn add_to_path(env: &impl Env, installation_dir: &Path) -> Result<()> {
    let installation_dir = installation_dir.to_str().unwrap();

    let path_var = env.path_var().unwrap_or_default();
    if !path_var.contains("bin") {
        info!("Make sure to have {installation_dir} in PATH");
    }

    Ok(())
}

async fn read_proxy_version(process: &impl Process, alias: &str) -> Result<Option<String>> {
    let version_arg = format!("--{}", env!("CARGO_BIN_NAME"));
    let output = match process.output(Path::new(alias), &[version_arg]).await {
        Ok(output) => output,
        Err(_) => return Ok(None),
    };

    let version = String::from_utf8(output)?.trim().to_string();
    if version.is_empty() {
        Ok(None)
    } else {
        Ok(Some(version))
    }
}
