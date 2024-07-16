use crate::{
    commands::install::{CardanoNode, Package},
    helpers::version::get_current_version,
};
use anyhow::{anyhow, Result};
use std::sync::{atomic::AtomicBool, Arc};
use tokio::time::{sleep, Duration};

pub async fn handle_proxy(rest_args: &[String]) -> miette::Result<()> {
    if !rest_args.is_empty() && rest_args[0].eq("--hyper-jump") {
        print!("hyper-jump v{}", env!("CARGO_PKG_VERSION"));
        return Ok(());
    }

    let package = Package::new_cardano_node("9.0.0".to_string());
    handle_package_process(rest_args, package).await.unwrap();

    Ok(())
}

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

    let mut child = tokio::process::Command::new(location);
    child.args(args);

    let mut spawned_child = child.spawn()?;

    watch_process(&mut spawned_child, &_term).await
}

async fn watch_process(
    spawned_child: &mut tokio::process::Child,
    term_signal: &Arc<AtomicBool>,
) -> Result<()> {
    tokio::select! {
        status = spawned_child.wait() => handle_process_exit(status).await,
        _ = tokio::signal::ctrl_c() => handle_ctrl_c(spawned_child, term_signal).await,
    }
}

async fn handle_process_exit(
    status: Result<std::process::ExitStatus, std::io::Error>,
) -> Result<()> {
    match status?.code() {
        Some(0) => Ok(()),
        Some(code) => Err(anyhow!("Process exited with error code {}", code)),
        None => Err(anyhow!("Process terminated by signal")),
    }
}

async fn handle_ctrl_c(
    spawned_child: &mut tokio::process::Child,
    term_signal: &Arc<AtomicBool>,
) -> Result<()> {
    term_signal.store(true, std::sync::atomic::Ordering::Relaxed);

    #[cfg(unix)]
    handle_unix_signals(spawned_child, term_signal)?;

    sleep(Duration::from_millis(200)).await;
    Ok(())
}

#[cfg(unix)]
fn handle_unix_signals(
    spawned_child: &mut tokio::process::Child,
    term_signal: &Arc<AtomicBool>,
) -> Result<()> {
    use nix::{
        sys::signal::{self, Signal},
        unistd::Pid,
    };
    use std::sync::atomic::Ordering;

    if term_signal.load(Ordering::Relaxed) {
        let pid = spawned_child.id().expect("Failed to get child process ID") as i32;
        signal::kill(Pid::from_raw(pid), Signal::SIGUSR1)?;
        term_signal.store(false, Ordering::Relaxed);
    }

    Ok(())
}
