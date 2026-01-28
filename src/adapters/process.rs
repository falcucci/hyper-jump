use std::path::Path;
use std::sync::atomic::AtomicBool;
use std::sync::Arc;

use anyhow::anyhow;
use anyhow::Result;
use tokio::time::sleep;
use tokio::time::Duration;

use crate::ports::Process;

#[derive(Clone, Copy)]
pub struct TokioProcess;

impl Process for TokioProcess {
    async fn run(&self, program: &Path, args: &[String]) -> anyhow::Result<()> {
        let _term = Arc::new(AtomicBool::new(false));

        #[cfg(unix)]
        {
            signal_hook::flag::register(signal_hook::consts::SIGUSR1, Arc::clone(&_term))?;
        }

        let mut child = tokio::process::Command::new(program);
        child.args(args);

        let mut spawned_child = child.spawn()?;
        watch_process(&mut spawned_child, &_term).await
    }

    async fn output(&self, program: &Path, args: &[String]) -> anyhow::Result<Vec<u8>> {
        let output = tokio::process::Command::new(program).args(args).output().await?;
        Ok(output.stdout)
    }
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
        Some(code) => {
            std::process::exit(code);
        }
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
