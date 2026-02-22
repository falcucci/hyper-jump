use std::path::Path;
use std::path::PathBuf;

use miette::IntoDiagnostic;

use crate::ports::Env;
use crate::ports::RootDir;

const DEFAULT_PATH_NAME: &str = "hyper-jump";

fn default_root_dir(env: &dyn Env) -> miette::Result<PathBuf> {
    let home = env.home_dir().ok_or_else(|| miette::miette!("Use root_dir parameter or env"))?;
    Ok(home.join(".local/share").join(DEFAULT_PATH_NAME))
}

pub fn ensure_root_dir(explicit: Option<&Path>, env: &dyn Env) -> miette::Result<PathBuf> {
    let defined = if let Some(path) = explicit {
        path.join(DEFAULT_PATH_NAME)
    } else if let Some(root) = env.root_dir() {
        root.join(DEFAULT_PATH_NAME)
    } else {
        default_root_dir(env)?
    };

    std::fs::create_dir_all(&defined).into_diagnostic()?;

    Ok(defined)
}

pub struct Dirs {
    pub root_dir: PathBuf,
}

impl Dirs {
    pub fn try_new(root_dir: Option<&Path>, env: &dyn Env) -> miette::Result<Self> {
        let root_dir = ensure_root_dir(root_dir, env)?;

        Ok(Self { root_dir })
    }
}

impl RootDir for Dirs {
    async fn root_dir(&self) -> anyhow::Result<PathBuf> { Ok(self.root_dir.clone()) }
}
