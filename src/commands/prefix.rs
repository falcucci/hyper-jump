use crate::ports::Paths;

/// Part of the `prefix` command, which is useful for adding
/// the installation directory to the PATH. The `prefix` command retrieves the
/// installation directory and prints it to the standard output.
pub async fn run(ctx: &crate::Context) -> miette::Result<()> {
    let paths = crate::adapters::path::FsPaths::new(ctx.dirs.root_dir.clone());
    let prefix = paths.installation_dir().await.map_err(|e| miette::miette!(e))?;

    println!("{}", prefix.display());

    Ok(())
}
