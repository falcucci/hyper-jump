use tracing::info;

use super::install::Package;
use crate::commands::install::install;
use crate::fs::copy_package_proxy;
use crate::helpers::version::is_version_used;
use crate::helpers::version::switch_version;
use crate::helpers::version::ParsedVersion;

pub async fn use_cmd(
    client: &reqwest::Client,
    version: ParsedVersion,
    package: Package,
) -> Result<(), Box<dyn std::error::Error>> {
    let is_version_used = is_version_used(&version.tag_name, package.clone()).await;

    println!("Copying package proxy...");
    copy_package_proxy(package.clone()).await?;

    if is_version_used {
        return Ok(());
    }

    println!("Installing version: {}", version.tag_name);
    install(client, package.clone(), version.clone()).await?;

    println!("Switching to version: {}", version.tag_name);
    switch_version(client, &version, package.clone()).await?;

    info!("You can now use {}!", version.tag_name);

    Ok(())
}
