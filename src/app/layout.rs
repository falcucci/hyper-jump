use crate::domain::package::PackageType;
use crate::ports::Platform;

pub fn binary_path(package_type: PackageType, platform: &impl Platform) -> String {
    let platform_name = platform.download(package_type.clone());
    let os = platform.os();
    match package_type {
        PackageType::CardanoSubmitApi => "bin".to_string(),
        PackageType::PartnerChainNode => "".to_string(),
        PackageType::SidechainCli => "".to_string(),
        PackageType::CardanoNode => "bin".to_string(),
        PackageType::CardanoCli => "bin".to_string(),
        PackageType::Jujutsu => "".to_string(),
        PackageType::Mithril => "".to_string(),
        PackageType::Zellij => "".to_string(),
        PackageType::Neovim => {
            format!(
                "nvim-{os}-{platform}/bin",
                os = os,
                platform = platform_name
            )
        }
        PackageType::Oura => "".to_string(),
        PackageType::Scrolls => "".to_string(),
        PackageType::Aiken => format!("aiken-{platform}", platform = platform_name),
        PackageType::Dolos => format!("dolos-{platform}", platform = platform_name),
        PackageType::Reth => "".to_string(),
    }
}
