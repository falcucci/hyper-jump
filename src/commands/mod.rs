use crate::helpers::version::LocalVersion;

pub mod erase;
pub mod install;
pub mod list;
pub mod list_remote;
pub mod prefix;
pub mod uninstall;
pub mod use_cmd;

/// Represents the type of a version after it has been downloaded.
///
/// This enum has three variants:
///
/// * `None` - No specific version type is assigned.
/// * `Standard(LocalVersion)` - The version is a standard version. The
///   `LocalVersion` contains the details of the version.
/// * `Hash` - The version is identified by a hash.
#[derive(PartialEq)]
pub enum PostDownloadVersionType {
    Standard(LocalVersion),
}
