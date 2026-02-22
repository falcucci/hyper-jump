use crate::domain::package::PackageSpec;
use crate::domain::version::parse_normal_version;
use crate::domain::version::ParsedVersion;
use crate::ports::ReleaseProvider;

pub async fn resolve_requested_version<R: ReleaseProvider>(
    requested: &str,
    spec: &PackageSpec,
    release_provider: &R,
) -> anyhow::Result<ParsedVersion> {
    if requested == "latest" {
        release_provider.latest(spec).await
    } else {
        parse_normal_version(requested).await
    }
}
