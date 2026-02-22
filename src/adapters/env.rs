use std::path::PathBuf;

use crate::ports::Env;

#[derive(Clone, Copy)]
pub struct StdEnv;

impl Env for StdEnv {
    fn exe_name(&self) -> String {
        std::env::args()
            .next()
            .as_deref()
            .and_then(|p| std::path::Path::new(p).file_stem())
            .and_then(|s| s.to_str())
            .unwrap_or_default()
            .to_string()
    }

    fn args(&self) -> Vec<String> { std::env::args().collect() }
    fn root_dir(&self) -> Option<PathBuf> {
        std::env::var_os("HYPER_JUMP_ROOT_DIR").map(PathBuf::from)
    }
    fn packages_file(&self) -> Option<PathBuf> {
        std::env::var_os("HYPER_JUMP_PACKAGES_FILE").map(PathBuf::from)
    }
    fn home_dir(&self) -> Option<PathBuf> { std::env::var_os("HOME").map(PathBuf::from) }
    fn current_exe(&self) -> anyhow::Result<PathBuf> { Ok(std::env::current_exe()?) }
    fn path_var(&self) -> Option<String> { std::env::var("PATH").ok() }
}
