use crate::{
  fs::copy_cardano_node_proxy,
  helpers::version::{is_version_used, ParsedVersion},
};

use super::install::Package;

pub async fn use_cmd(
  client: &reqwest::Client,
  version: ParsedVersion,
  package: Package,
) -> Result<(), Box<dyn std::error::Error>> {
  let is_version_used = is_version_used(&version.tag_name, package.clone()).await;
  println!("is_version_used: {:?}", is_version_used);

  copy_cardano_node_proxy(package).await?;

  Ok(())
}
