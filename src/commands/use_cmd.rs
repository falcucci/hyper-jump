use crate::helpers::version::{is_version_used, ParsedVersion};

use super::install::Package;

pub async fn use_cmd(
  client: &reqwest::Client,
  version: ParsedVersion,
  package: Package,
) -> Result<(), Box<dyn std::error::Error>> {
  let is_version_used = is_version_used(&version.tag_name, package).await;
  println!("is_version_used: {:?}", is_version_used);

  Ok(())
}
