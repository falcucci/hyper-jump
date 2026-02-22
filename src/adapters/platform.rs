use crate::domain::package::PackageType;
use crate::ports::Platform;

pub struct StdPlatform;

impl Platform for StdPlatform {
    fn os(&self) -> &'static str { std::env::consts::OS }

    fn file_type(&self, package_type: PackageType) -> &'static str {
        #[cfg(target_family = "windows")]
        {
            "zip"
        }

        #[cfg(target_os = "macos")]
        {
            match package_type {
                PackageType::CardanoSubmitApi => "tar.gz",
                PackageType::PartnerChainNode => "zip",
                PackageType::SidechainCli => "zip",
                PackageType::CardanoNode => "tar.gz",
                PackageType::CardanoCli => "tar.gz",
                PackageType::Jujutsu => "tar.gz",
                PackageType::Mithril => "tar.gz",
                PackageType::Scrolls => "tar.gz",
                PackageType::Aiken => "tar.gz",
                PackageType::Zellij => "tar.gz",
                PackageType::Neovim => "tar.gz",
                PackageType::Dolos => "tar.xz",
                PackageType::Oura => "tar.gz",
                PackageType::Reth => "tar.gz",
            }
        }

        #[cfg(target_os = "linux")]
        {
            match package_type {
                PackageType::CardanoSubmitApi => "tar.gz",
                PackageType::PartnerChainNode => "zip",
                PackageType::SidechainCli => "zip",
                PackageType::CardanoNode => "tar.gz",
                PackageType::CardanoCli => "tar.gz",
                PackageType::Jujutsu => "tar.gz",
                PackageType::Mithril => "tar.gz",
                PackageType::Scrolls => "tar.gz",
                PackageType::Zellij => "tar.gz",
                PackageType::Neovim => "tar.gz",
                PackageType::Aiken => "tar.gz",
                PackageType::Dolos => "tar.xz",
                PackageType::Oura => "tar.gz",
                PackageType::Reth => "tar.gz",
            }
        }
    }

    fn download(&self, package_type: PackageType) -> &'static str {
        #[cfg(target_family = "windows")]
        {
            "win64"
        }

        #[cfg(target_os = "macos")]
        {
            #[cfg(target_arch = "aarch64")]
            {
                match package_type {
                    PackageType::CardanoSubmitApi => "",
                    PackageType::PartnerChainNode => "arm64",
                    PackageType::SidechainCli => "arm64",
                    PackageType::CardanoNode => "",
                    PackageType::CardanoCli => "",
                    PackageType::Jujutsu => "aarch64-apple-darwin",
                    PackageType::Mithril => "arm64",
                    PackageType::Scrolls => "aarch64-apple-darwin",
                    PackageType::Aiken => "aarch64-apple-darwin",
                    PackageType::Dolos => "aarch64-apple-darwin",
                    PackageType::Zellij => "aarch64-apple-darwin",
                    PackageType::Neovim => "arm64",
                    PackageType::Oura => "aarch64-apple-darwin",
                    PackageType::Reth => "aarch64-apple-darwin",
                }
            }

            #[cfg(target_arch = "x86_64")]
            {
                match package_type {
                    PackageType::CardanoSubmitApi => "",
                    PackageType::PartnerChainNode => "x86_64",
                    PackageType::SidechainCli => "x86_64",
                    PackageType::CardanoNode => "",
                    PackageType::CardanoCli => "",
                    PackageType::Jujutsu => "x86_64-apple-darwin",
                    PackageType::Mithril => "x86_64",
                    PackageType::Scrolls => "x86_64",
                    PackageType::Aiken => "x86_64-apple-darwin",
                    PackageType::Dolos => "x86_64-apple-darwin",
                    PackageType::Zellij => "x86_64-apple-darwin",
                    PackageType::Neovim => "x86_64",
                    PackageType::Oura => "x86_64-apple-darwin",
                    PackageType::Reth => "x86_64-apple-darwin",
                }
            }
        }

        #[cfg(target_os = "linux")]
        {
            match package_type {
                PackageType::CardanoSubmitApi => "",
                PackageType::PartnerChainNode => "",
                PackageType::SidechainCli => "",
                PackageType::CardanoNode => "",
                PackageType::CardanoCli => "",
                PackageType::Jujutsu => "x86_64-unknown-linux-musl",
                PackageType::Mithril => "x64",
                PackageType::Scrolls => "x64",
                PackageType::Aiken => "x86_64-unknown-linux-gnu",
                PackageType::Dolos => "x86_64-unknown-linux-gnu",
                PackageType::Zellij => "x86_64-unknown-linux-gnu",
                PackageType::Neovim => "x86_64",
                PackageType::Oura => "x86_64-unknown-linux-musl",
                PackageType::Reth => "x86_64-unknown-linux-gnu",
            }
        }
    }
}
