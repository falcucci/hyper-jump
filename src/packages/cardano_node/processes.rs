use std::{
    sync::{atomic::AtomicBool, Arc},
    time::Duration,
};

use anyhow::{anyhow, Result};
use tokio::time::sleep;

use crate::{
    commands::install::{CardanoNode, Package},
    helpers::version::get_current_version,
};

/// Handles the execution process.
///
/// It retrieves the downloads directory and the currently used version from the configuration.
/// It then constructs the path to the binary and spawns a new process with the given arguments.
/// The function then enters a loop where it continuously checks the status of the spawned process.
/// If the process exits with a status code of `0`, the function returns `Ok(())`.
/// If the process exits with a non-zero status code, the function returns an error with the status code as the error message.
/// If the process is terminated by a signal, the function returns an error with the message "Process terminated by signal".
/// If the function fails to wait on the child process, it returns an error with the message "Failed to wait on child process".
///
/// # Arguments
///
/// * `args` - A slice of `String` arguments to be passed to the process.
///
/// # Returns
///
/// This function returns a `Result` that indicates whether the operation was successful.
/// If the operation was successful, the function returns `Ok(())`.
/// If the operation failed, the function returns `Err` with a description of the error.
///
/// # Errors
///
/// This function will return an error if:
///
/// * The process exits with a non-zero status code.
/// * The process is terminated by a signal.
/// * The function fails to wait on the child process.
///
/// # Example
///
/// ```rust
/// let args = vec!["-v".to_string()];
/// handle_package_process(&args).await;
/// ```
pub async fn handle_package_process(args: &[String], package: Package) -> Result<()> {
    let downloads_dir = crate::fs::get_downloads_directory(package.clone()).await?;
    let used_version = get_current_version(package.clone()).await?;

    let alias = match package {
        Package::CardanoNode(CardanoNode { alias, .. }) => alias,
        Package::CardanoCli => todo!(),
        Package::Mithril => todo!(),
    };

    let location = downloads_dir.join(used_version).join("bin").join(alias);
    println!("Running: {:?}", location);

    let _term = Arc::new(AtomicBool::new(false));

    #[cfg(unix)]
    {
        signal_hook::flag::register(signal_hook::consts::SIGUSR1, Arc::clone(&_term))?;
    }

    let mut child = std::process::Command::new(location);
    child.args(args);

    let mut spawned_child = child.spawn()?;
    loop {
        let child_done = spawned_child.try_wait();
        match child_done {
            Ok(Some(status)) => match status.code() {
                Some(0) => return Ok(()),
                Some(code) => return Err(anyhow!("Process exited with error code {}", code)),
                None => return Err(anyhow!("Process terminated by signal")),
            },
            Ok(None) => {
                #[cfg(unix)]
                {
                    use nix::sys::signal::{self, Signal};
                    use nix::unistd::Pid;
                    use std::sync::atomic::Ordering;
                    if _term.load(Ordering::Relaxed) {
                        let pid = spawned_child.id() as i32;
                        signal::kill(Pid::from_raw(pid), Signal::SIGUSR1)?;
                        _term.store(false, Ordering::Relaxed);
                    }
                }
                // short delay to a void high cpu usage
                sleep(Duration::from_millis(200)).await;
            }
            Err(_) => return Err(anyhow!("Failed to wait on child process")),
        }
    }
}
