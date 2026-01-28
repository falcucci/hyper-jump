use crate::domain::package::Package;
use crate::ports::Platform;

pub fn download_url(package: &Package, platform: &impl Platform) -> String {
    let version = package.version().expect("Version not set");
    let package_type = package.package_type();

    package
        .get_template_url()
        .replace("{version}", version.non_parsed_string.as_str())
        .replace("{OS}", platform.os())
        .replace("{platform}", platform.download(package_type.clone()))
        .replace("{file_type}", platform.file_type(package_type))
}
