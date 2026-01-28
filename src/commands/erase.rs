use crate::adapters::fs::TokioFs;
use crate::app::erase::erase;

/// Asynchronously erases the hyper-jump installation and downloads folders.
///
/// This function attempts to remove the hyper-jump installation directory and
/// the downloads directory. It logs successful removals and returns an error if
/// there is nothing to erase or if an error occurs during the removal process.
///
/// # Errors
///
/// Returns an error if both the installation and downloads directories do not
/// exist or cannot be removed.
pub async fn run(ctx: &crate::Context) -> miette::Result<()> {
    let fs = TokioFs;
    erase(&ctx.dirs, &fs).await.map_err(|e| miette::miette!(e))
}
