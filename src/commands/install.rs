use std::env;

use anyhow::Error;
use reqwest::Client;

use crate::{
  fs::get_downloads_directory,
  helpers::version::{is_version_installed, ParsedVersion},
};

#[derive(Debug)]
pub enum Package {
  CardanoNode,
  CardanoCli,
  Mithril,
}

pub async fn install(
  client: &Client,
  package: Package,
  version: ParsedVersion,
) -> Result<(), Error> {
  println!("installing package: {:?}", package);
  let root = get_downloads_directory().await?;

  env::set_current_dir(&root)?;
  let root = root.as_path();

  println!("version: {:?}", version);

  let is_version_installed = is_version_installed(&version.tag_name).await?;
  println!("is_version_installed: {:?}", is_version_installed);

  // let package = package::Package::new(package, version)?;
  // let package = package.resolve(client)?;
  //
  // let package_dir = dirs::package_dir(&package.name, &package.version);
  // let package_dir = package_dir.as_path();
  //
  // if package_dir.exists() {
  //   return Err(Error::PackageAlreadyInstalled {
  //     package: package.name.clone(),
  //     version: package.version.clone(),
  //   });
  // }
  //
  // let package_tarball = dirs::package_tarball(&package.name, &package.version);
  // let package_tarball = package_tarball.as_path();
  //
  // let package_tarball_url = package.tarball_url();
  // let package_tarball_url = package_tarball_url.as_str();
  //
  // let package_tarball_response = client.get(package_tarball_url).send()?;
  // let package_tarball_response = package_tarball_response.error_for_status()?;
  //
  // let package_tarball_bytes = package_tarball_response.bytes()?;
  // let package_tarball_bytes = package_tarball_bytes.as_ref();
  //
  // fs::create_dir_all(package_dir)?;
  // fs::write(package_tarball, package_tarball_bytes)?;
  //
  // let package_tarball = fs::File::open(package_tarball)?;
  // let package_tarball = flate2::read::GzDecoder::new(package_tarball);
  // let package_tarball = tar::Archive::new(package_tarball);
  //
  // package_tarball.unpack(package_dir)?;

  Ok(())
}
