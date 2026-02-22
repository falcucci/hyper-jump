use std::collections::HashMap;
use std::path::Path;
use std::path::PathBuf;
use std::sync::Arc;

use anyhow::anyhow;
use anyhow::Context;
use anyhow::Result;
use config::Config;
use config::File;
use config::FileFormat;
use serde::Deserialize;

use crate::domain::version::ParsedVersion;
use crate::ports::Platform;

const DEFAULT_BASE_URL: &str = "https://github.com";
const DEFAULT_API_BASE_URL: &str = "https://api.github.com/repos";

fn default_base_url() -> String { DEFAULT_BASE_URL.to_string() }
fn default_api_base_url() -> String { DEFAULT_API_BASE_URL.to_string() }

#[derive(Debug, Clone, Deserialize)]
pub struct PackageSpec {
    pub id: String,
    pub alias: String,
    pub repo: String,
    pub download_template: String,
    #[serde(default)]
    pub binary_path_template: String,
    #[serde(default = "default_base_url")]
    pub base_url: String,
    #[serde(default = "default_api_base_url")]
    pub api_base_url: String,
    pub platform: PlatformMatrix,
    pub ext: ExtMatrix,
}

#[derive(Debug, Clone, Deserialize)]
struct RawPackageSpec {
    pub id: String,
    pub alias: String,
    pub repo: String,
    pub download_template: String,
    #[serde(default)]
    pub binary_path_template: String,
    pub base_url: Option<String>,
    pub api_base_url: Option<String>,
    pub platform: Option<PlatformMatrix>,
    pub ext: Option<ExtMatrix>,
}

#[derive(Debug, Clone, Deserialize, Default)]
struct PackageDefaults {
    pub base_url: Option<String>,
    pub api_base_url: Option<String>,
    pub platform: Option<PlatformMatrix>,
    pub ext: Option<ExtMatrix>,
}

#[derive(Debug, Clone, Deserialize)]
pub struct PlatformMatrix {
    pub macos: Option<ArchMatrix>,
    pub linux: Option<ArchMatrix>,
    pub windows: Option<ArchMatrix>,
}

#[derive(Debug, Clone, Deserialize)]
pub struct ArchMatrix {
    pub aarch64: Option<String>,
    pub x86_64: Option<String>,
}

#[derive(Debug, Clone, Deserialize)]
pub struct ExtMatrix {
    pub macos: Option<String>,
    pub linux: Option<String>,
    pub windows: Option<String>,
}

impl PackageSpec {
    pub fn latest_url(&self) -> String {
        format!("{}/{}/releases/latest", self.api_base_url, self.repo)
    }

    pub fn releases_url(&self) -> String { format!("{}/{}/releases", self.api_base_url, self.repo) }

    pub fn download_url(
        &self,
        version: &ParsedVersion,
        platform: &impl Platform,
    ) -> Result<String> {
        let platform_tag = self.platform_tag(platform)?;
        let file_type = self.file_type(platform)?;
        Ok(self
            .download_template
            .replace("{base}", &self.base_url)
            .replace("{repo}", &self.repo)
            .replace("{version}", version.non_parsed_string.as_str())
            .replace("{OS}", platform.os())
            .replace("{platform}", &platform_tag)
            .replace("{file_type}", &file_type))
    }

    pub fn binary_path(&self, platform: &impl Platform) -> Result<String> {
        if self.binary_path_template.is_empty() {
            return Ok(String::new());
        }

        let platform_tag = self.platform_tag(platform)?;
        Ok(self
            .binary_path_template
            .replace("{OS}", platform.os())
            .replace("{platform}", &platform_tag))
    }

    fn platform_tag(&self, platform: &impl Platform) -> Result<String> {
        let os = platform.os();
        let arch = platform.arch();
        let arch_map = match os {
            "macos" => self.platform.macos.as_ref(),
            "linux" => self.platform.linux.as_ref(),
            "windows" => self.platform.windows.as_ref(),
            _ => None,
        }
        .ok_or_else(|| anyhow!("No platform mapping for OS '{os}'"))?;

        let value = match arch {
            "aarch64" => arch_map.aarch64.as_ref(),
            "x86_64" => arch_map.x86_64.as_ref(),
            _ => None,
        }
        .ok_or_else(|| anyhow!("No platform mapping for arch '{arch}'"))?;

        Ok(value.clone())
    }

    pub fn file_type(&self, platform: &impl Platform) -> Result<String> {
        let os = platform.os();
        let value = match os {
            "macos" => self.ext.macos.as_ref(),
            "linux" => self.ext.linux.as_ref(),
            "windows" => self.ext.windows.as_ref(),
            _ => None,
        }
        .ok_or_else(|| anyhow!("No file type for OS '{os}'"))?;

        Ok(value.clone())
    }
}

#[derive(Debug, Deserialize)]
struct PackagesFile {
    #[serde(default)]
    defaults: PackageDefaults,
    #[serde(default)]
    package: Vec<RawPackageSpec>,
    #[serde(default)]
    org: Vec<OrgBlock>,
}

#[derive(Debug, Clone, Deserialize)]
struct OrgBlock {
    pub name: String,
    #[serde(default)]
    pub defaults: PackageDefaults,
    pub package: Vec<RawPackageSpec>,
}

#[derive(Debug, Clone)]
struct PackageEntry {
    spec: RawPackageSpec,
    defaults: PackageDefaults,
    org: Option<String>,
}

#[derive(Clone)]
pub struct PackageRegistry {
    by_id: HashMap<String, Arc<PackageSpec>>,
    by_alias: HashMap<String, Arc<PackageSpec>>,
}

impl PackageRegistry {
    pub fn load_from_paths(
        explicit_path: Option<PathBuf>,
        default_path: PathBuf,
        fallback: &str,
    ) -> Result<Self> {
        if let Some(path) = explicit_path {
            return Self::load_from_path(&path)
                .with_context(|| format!("Failed to load packages from {}", path.display()));
        }

        if default_path.exists() {
            return Self::load_from_path(&default_path).with_context(|| {
                format!("Failed to load packages from {}", default_path.display())
            });
        }

        Self::load_from_str(fallback).context("Failed to load embedded packages")
    }

    pub fn load_from_path(path: &Path) -> Result<Self> {
        let contents = std::fs::read_to_string(path)?;
        Self::load_from_str(&contents)
    }

    pub fn load_from_str(contents: &str) -> Result<Self> {
        let config = Config::builder()
            .add_source(File::from_str(contents, FileFormat::Toml))
            .build()?;
        let parsed: PackagesFile = config.try_deserialize()?;
        let mut entries = Vec::new();

        for spec in parsed.package {
            entries.push(PackageEntry {
                spec,
                defaults: parsed.defaults.clone(),
                org: None,
            });
        }

        for org in parsed.org {
            if org.name.trim().is_empty() {
                return Err(anyhow!("Org name cannot be empty"));
            }
            let defaults = merge_defaults(&parsed.defaults, &org.defaults);
            for spec in org.package {
                entries.push(PackageEntry {
                    spec,
                    defaults: defaults.clone(),
                    org: Some(org.name.clone()),
                });
            }
        }

        Self::from_entries(entries)
    }

    pub fn resolve(&self, name: &str) -> Result<Arc<PackageSpec>> {
        if let Some(spec) = self.by_id.get(name) {
            return Ok(spec.clone());
        }
        if let Some(spec) = self.by_alias.get(name) {
            return Ok(spec.clone());
        }

        Err(anyhow!(
            "Unknown package '{name}'. Available: {}",
            self.ids().join(", ")
        ))
    }

    pub fn get_by_alias(&self, alias: &str) -> Result<Arc<PackageSpec>> {
        self.by_alias
            .get(alias)
            .cloned()
            .ok_or_else(|| anyhow!("Unknown package alias '{alias}'"))
    }

    pub fn ids(&self) -> Vec<String> {
        let mut ids: Vec<String> = self.by_id.keys().cloned().collect();
        ids.sort();
        ids
    }

    fn from_entries(entries: Vec<PackageEntry>) -> Result<Self> {
        let mut by_id = HashMap::new();
        let mut by_alias = HashMap::new();

        for entry in entries {
            let spec = entry.spec;
            if spec.id.trim().is_empty() {
                return Err(anyhow!("Package id cannot be empty"));
            }

            if spec.alias.trim().is_empty() {
                return Err(anyhow!("Package alias cannot be empty"));
            }

            if spec.repo.trim().is_empty() {
                return Err(anyhow!("Package repo cannot be empty"));
            }

            let spec = Arc::new(spec.into_spec(&entry.defaults, entry.org.as_deref())?);
            if by_id.insert(spec.id.clone(), spec.clone()).is_some() {
                return Err(anyhow!("Duplicate package id '{}'", spec.id));
            }

            if let Some(existing) = by_id.get(&spec.alias) {
                if !Arc::ptr_eq(existing, &spec) {
                    return Err(anyhow!(
                        "Alias '{}' conflicts with package id '{}'",
                        spec.alias,
                        existing.id
                    ));
                }
            }

            if by_alias.insert(spec.alias.clone(), spec.clone()).is_some() {
                return Err(anyhow!("Duplicate package alias '{}'", spec.alias));
            }
        }

        Ok(Self { by_id, by_alias })
    }
}

impl RawPackageSpec {
    fn into_spec(self, defaults: &PackageDefaults, org: Option<&str>) -> Result<PackageSpec> {
        let base_url = self
            .base_url
            .or_else(|| defaults.base_url.clone())
            .unwrap_or_else(default_base_url);
        let api_base_url = self
            .api_base_url
            .or_else(|| defaults.api_base_url.clone())
            .unwrap_or_else(default_api_base_url);
        let platform = merge_platform(self.platform, defaults.platform.clone())
            .ok_or_else(|| anyhow!("Package '{}' missing platform mapping", self.id))?;
        let ext = merge_ext(self.ext, defaults.ext.clone())
            .ok_or_else(|| anyhow!("Package '{}' missing file type mapping", self.id))?;
        let repo = if self.repo.contains('/') {
            self.repo
        } else if let Some(org) = org {
            format!("{org}/{}", self.repo)
        } else {
            self.repo
        };

        Ok(PackageSpec {
            id: self.id,
            alias: self.alias,
            repo,
            download_template: self.download_template,
            binary_path_template: self.binary_path_template,
            base_url,
            api_base_url,
            platform,
            ext,
        })
    }
}

fn merge_platform(
    spec: Option<PlatformMatrix>,
    defaults: Option<PlatformMatrix>,
) -> Option<PlatformMatrix> {
    match (spec, defaults) {
        (Some(mut spec), Some(defaults)) => {
            spec.macos = merge_arch(spec.macos, defaults.macos);
            spec.linux = merge_arch(spec.linux, defaults.linux);
            spec.windows = merge_arch(spec.windows, defaults.windows);
            Some(spec)
        }
        (Some(spec), None) => Some(spec),
        (None, Some(defaults)) => Some(defaults),
        (None, None) => None,
    }
}

fn merge_arch(spec: Option<ArchMatrix>, defaults: Option<ArchMatrix>) -> Option<ArchMatrix> {
    match (spec, defaults) {
        (Some(mut spec), Some(defaults)) => {
            spec.aarch64 = spec.aarch64.or(defaults.aarch64);
            spec.x86_64 = spec.x86_64.or(defaults.x86_64);
            Some(spec)
        }
        (Some(spec), None) => Some(spec),
        (None, Some(defaults)) => Some(defaults),
        (None, None) => None,
    }
}

fn merge_ext(spec: Option<ExtMatrix>, defaults: Option<ExtMatrix>) -> Option<ExtMatrix> {
    match (spec, defaults) {
        (Some(mut spec), Some(defaults)) => {
            spec.macos = spec.macos.or(defaults.macos);
            spec.linux = spec.linux.or(defaults.linux);
            spec.windows = spec.windows.or(defaults.windows);
            Some(spec)
        }
        (Some(spec), None) => Some(spec),
        (None, Some(defaults)) => Some(defaults),
        (None, None) => None,
    }
}

fn merge_defaults(base: &PackageDefaults, overlay: &PackageDefaults) -> PackageDefaults {
    PackageDefaults {
        base_url: overlay.base_url.clone().or_else(|| base.base_url.clone()),
        api_base_url: overlay.api_base_url.clone().or_else(|| base.api_base_url.clone()),
        platform: merge_platform(overlay.platform.clone(), base.platform.clone()),
        ext: merge_ext(overlay.ext.clone(), base.ext.clone()),
    }
}

#[derive(Clone)]
pub struct Package {
    spec: Arc<PackageSpec>,
    version: Option<ParsedVersion>,
    binary_path: String,
}

impl Package {
    pub fn from_spec(spec: Arc<PackageSpec>, platform: &impl Platform) -> Result<Self> {
        let binary_path = spec.binary_path(platform)?;
        Ok(Self {
            spec,
            version: None,
            binary_path,
        })
    }

    pub fn with_parsed(
        spec: Arc<PackageSpec>,
        version: ParsedVersion,
        platform: &impl Platform,
    ) -> Result<Self> {
        let binary_path = spec.binary_path(platform)?;
        Ok(Self {
            spec,
            version: Some(version),
            binary_path,
        })
    }

    pub fn spec(&self) -> &PackageSpec { &self.spec }
    pub fn alias(&self) -> String { self.spec.alias.clone() }
    pub fn version(&self) -> Option<ParsedVersion> { self.version.clone() }
    pub fn binary_path(&self) -> String { self.binary_path.clone() }
    pub fn binary_name(&self) -> String { self.spec.alias.clone() }
}
