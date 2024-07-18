use std::sync::atomic::AtomicBool;
use std::sync::Arc;

use anyhow::anyhow;
use anyhow::Result;
use tokio::time::sleep;
use tokio::time::Duration;

use crate::commands::install::CardanoCli;
use crate::commands::install::CardanoNode;
use crate::commands::install::Package;
use crate::helpers::version::get_current_version;

/// Handles the proxy command with optional arguments.
///
/// This function processes the provided arguments and executes the appropriate
/// action based on the input. If the first argument is `--hyper-jump`, it
/// prints the version information of itself. Otherwise, it constructs a new
/// `Package` to processes it.
///
/// # Arguments
///
/// * `rest_args` - A slice of strings containing the command-line arguments.
///
/// # Returns
///
/// This function returns a `Result` indicating the success or failure of the
/// operation.
///
/// * `Ok(())` - The operation was successful.
/// * `Err(miette::Error)` - An error occurred during the operation.
///
/// # Examples
///
/// ```rust
/// let args = vec!["some-other-arg".to_string()];
/// handle_proxy(&args).await?;
/// ```
///
/// # Errors
///
/// This function will return an error if the `handle_package_process` function
/// fails.
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
/// It retrieves the downloads directory and the currently used version from the
/// configuration. It then constructs the path to the binary and spawns a new
/// process with the given arguments. The function then enters a loop where it
/// continuously checks the status of the spawned process. If the process exits
/// with a status code of `0`, the function returns `Ok(())`. If the process
/// exits with a non-zero status code, the function returns an error with the
/// status code as the error message. If the process is terminated by a signal,
/// the function returns an error with the message "Process terminated by
/// signal". If the function fails to wait on the child process, it returns an
/// error with the message "Failed to wait on child process".
///
/// # Arguments
///
/// * `args` - A slice of `String` arguments to be passed to the process.
///
/// # Returns
///
/// This function returns a `Result` that indicates whether the operation was
/// successful. If the operation was successful, the function returns `Ok(())`.
/// If the operation failed, the function returns `Err` with a description of
/// the error.
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
        Package::CardanoCli(CardanoCli { alias, .. }) => alias,
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

/// Watches a spawned child process and handles termination signals.
///
/// This function concurrently waits for the spawned child process to exit or
/// for a Ctrl-C signal to be received. It handles each scenario appropriately.
///
/// # Arguments
///
/// * `spawned_child` - A mutable reference to the spawned child process.
/// * `term_signal` - An `Arc` containing an `AtomicBool` used to signal
///   termination.
///
/// # Returns
///
/// This function returns a `Result` indicating the success or failure of the
/// operation.
///
/// * `Ok(())` - The operation was successful.
/// * `Err(anyhow::Error)` - An error occurred during the operation.
///
/// # Errors
///
/// This function will return an error if either `handle_process_exit` or
/// `handle_ctrl_c` encounters an error.
///
/// # Examples
///
/// ```rust
/// # async fn example() -> Result<()> {
/// let term_signal = Arc::new(AtomicBool::new(false));
/// let mut child = tokio::process::Command::new("some_command").spawn()?;
/// watch_process(&mut child, &term_signal).await?.
/// ```
async fn watch_process(
    spawned_child: &mut tokio::process::Child,
    term_signal: &Arc<AtomicBool>,
) -> Result<()> {
    tokio::select! {
        status = spawned_child.wait() => handle_process_exit(status).await,
        _ = tokio::signal::ctrl_c() => handle_ctrl_c(spawned_child, term_signal).await,
    }
}

/// Handles the exit of a spawned child process.
///
/// This function processes the exit status of the child process and returns an
/// appropriate result based on the exit code.
///
/// # Arguments
///
/// * `status` - The exit status of the child process.
///
/// # Returns
///
/// This function returns a `Result` indicating the success or failure of the
/// operation.
///
/// * `Ok(())` - The process exited successfully.
/// * `Err(anyhow::Error)` - The process exited with an error code or was
///   terminated by a signal.
///
/// # Errors
///
/// This function will return an error if the process exited with a non-zero
/// exit code or was terminated by a signal.
///
/// # Examples
///
/// ```rust
/// let status = Ok(std::process::ExitStatus::from_raw(0));
/// handle_process_exit(status).await?;
/// ```
async fn handle_process_exit(
    status: Result<std::process::ExitStatus, std::io::Error>,
) -> Result<()> {
    match status?.code() {
        Some(0) => Ok(()),
        Some(code) => Err(anyhow!("Process exited with error code {}", code)),
        None => Err(anyhow!("Process terminated by signal")),
    }
}

/// Handles the Ctrl-C signal.
///
/// This function sets the termination signal and handles Unix-specific signals
/// if applicable.
///
/// # Arguments
///
/// * `spawned_child` - A mutable reference to the spawned child process.
/// * `term_signal` - An `Arc` containing an `AtomicBool` used to signal
///   termination.
///
/// # Returns
///
/// This function returns a `Result` indicating the success or failure of the
/// operation.
///
/// * `Ok(())` - The operation was successful.
/// * `Err(anyhow::Error)` - An error occurred during the operation.
///
/// # Errors
///
/// This function will return an error if `handle_unix_signals` encounters an
/// error.
///
/// # Examples
///
/// ```rust
/// let term_signal = Arc::new(AtomicBool::new(false));
/// let mut child = tokio::process::Command::new("some_command").spawn()?;
/// handle_ctrl_c(&mut child, &term_signal).await?;
/// ```
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

/// Handles Unix-specific termination signals.
///
/// This function sends a Unix signal to the spawned child process if the
/// termination signal is set.
///
/// # Arguments
///
/// * `spawned_child` - A mutable reference to the spawned child process.
/// * `term_signal` - An `Arc` containing an `AtomicBool` used to signal
///   termination.
///
/// # Returns
///
/// This function returns a `Result` indicating the success or failure of the
/// operation.
///
/// * `Ok(())` - The operation was successful.
/// * `Err(anyhow::Error)` - An error occurred during the operation.
///
/// # Errors
///
/// This function will return an error if it fails to send the Unix signal.
///
/// # Examples
///
/// ```rust
/// let term_signal = Arc::new(AtomicBool::new(true));
/// let mut child = tokio::process::Command::new("some_command").spawn()?;
/// handle_unix_signals(&mut child, &term_signal)?;
/// ```
#[cfg(unix)]
fn handle_unix_signals(
    spawned_child: &mut tokio::process::Child,
    term_signal: &Arc<AtomicBool>,
) -> Result<()> {
    use std::sync::atomic::Ordering;

    use nix::sys::signal::Signal;
    use nix::sys::signal::{self};
    use nix::unistd::Pid;

    if term_signal.load(Ordering::Relaxed) {
        let pid = spawned_child.id().expect("Failed to get child process ID") as i32;
        signal::kill(Pid::from_raw(pid), Signal::SIGUSR1)?;
        term_signal.store(false, Ordering::Relaxed);
    }

    Ok(())
}
