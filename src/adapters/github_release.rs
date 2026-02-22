use anyhow::Result;
use serde::Deserialize;

use crate::adapters::github::api;
use crate::adapters::github::deserialize_response;
use crate::domain::package::PackageType;
use crate::domain::version::parse_normal_version;
use crate::domain::version::ParsedVersion;
use crate::domain::version::RemoteVersion;
use crate::ports::ReleaseProvider;

pub struct GitHubReleaseProvider {
    client: Option<reqwest::Client>,
}

#[derive(Debug, Deserialize)]
struct UpstreamVersion {
    pub tag_name: String,
}

impl GitHubReleaseProvider {
    pub fn new(client: Option<&reqwest::Client>) -> Self {
        Self {
            client: client.cloned(),
        }
    }
}

impl ReleaseProvider for GitHubReleaseProvider {
    async fn latest(&self, package: PackageType) -> Result<ParsedVersion> {
        let url = package.get_latest_url();
        let response = api(self.client.as_ref(), url).await?;
        let latest: UpstreamVersion = deserialize_response(response)?;
        parse_normal_version(&latest.tag_name).await
    }

    async fn list(&self, package: PackageType) -> Result<Vec<RemoteVersion>> {
        let url = format!("{}/{}/releases", package.api_base_url(), package.repo());
        let response = api(self.client.as_ref(), url).await?;
        let versions: Vec<RemoteVersion> = deserialize_response(response)?;
        Ok(versions)
    }
}
