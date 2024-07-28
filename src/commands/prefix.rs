/// Part of the `prefix` command, which is useful for adding
/// the installation directory to the PATH. The `prefix` command retrieves the
/// installation directory and prints it to the standard output.
pub async fn run() -> miette::Result<()> {
    let prefix = crate::fs::get_installation_directory()
        .await
        .expect("Failed to get installation directory");

    println!("{}", prefix.display());

    Ok(())
}
