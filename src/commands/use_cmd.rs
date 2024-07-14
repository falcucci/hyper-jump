use crate::helpers::version::{is_version_used, ParsedVersion};

pub async fn use_cmd(
  client: &reqwest::Client,
  version: ParsedVersion,
) -> Result<(), Box<dyn std::error::Error>> {
  let is_version_used = is_version_used(&version.tag_name).await;
  println!("is_version_used: {:?}", is_version_used);

  Ok(())
}
