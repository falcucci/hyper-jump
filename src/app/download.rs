use crate::domain::package::Package;
use crate::ports::Platform;

pub fn download_url(package: &Package, platform: &impl Platform) -> String {
    package
        .spec()
        .download_url(&package.version().expect("Version not set"), platform)
        .expect("Invalid download template")
}
