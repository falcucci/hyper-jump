use std::path::PathBuf;

use anyhow::Error;
use anyhow::Result;
use comfy_table::modifiers::UTF8_ROUND_CORNERS;
use comfy_table::presets::UTF8_FULL;
use comfy_table::Cell;
use comfy_table::CellAlignment;
use comfy_table::Color;
use comfy_table::Table;
use serde::Serialize;
use tracing::info;

use crate::domain::package::Package;
use crate::ports::Fs;
use crate::ports::Output;
use crate::ports::Paths;
use crate::ports::UsedVersionStore;
use crate::OutputFormat;

#[derive(Serialize)]
struct ListedVersion {
    version: String,
    status: String,
}

pub async fn list_installed(
    package: Package,
    fmt: OutputFormat,
    output: &impl Output,
    paths: &impl Paths,
    used_store: &impl UsedVersionStore,
    fs: &impl Fs,
) -> Result<(), Error> {
    let downloads_dir = paths.downloads_dir(package.clone()).await?;

    let entries: Vec<PathBuf> = fs.read_dir(&downloads_dir).await?;

    if entries.is_empty() {
        info!("There are no versions installed");
        return Ok(());
    }

    let current = used_store.current(package.clone()).await?;
    let current_norm = current.as_deref().map(normalize_tag);

    match fmt {
        OutputFormat::Json => {
            let mut rows = Vec::new();
            for path in entries {
                if !fs.is_dir(&path).await? {
                    continue;
                }
                let path_name = path.file_name().unwrap_or_default().to_string_lossy().to_string();
                let path_norm = normalize_tag(&path_name);
                let status = match current_norm {
                    Some(cur) if cur == path_norm => "used",
                    _ => "installed",
                };
                rows.push(ListedVersion {
                    version: path_name,
                    status: status.to_string(),
                });
            }
            let json = serde_json::to_string_pretty(&rows)?;
            output.write_line(&json)?;
        }
        OutputFormat::Table => {
            let mut table = Table::new();
            let header = vec!["Version", "Status"];
            table.load_preset(UTF8_FULL).apply_modifier(UTF8_ROUND_CORNERS);
            table.set_header(header);

            for path in entries {
                if !fs.is_dir(&path).await? {
                    continue;
                }

                let path_name = path.file_name().unwrap_or_default().to_string_lossy().to_string();
                let path_norm = normalize_tag(&path_name);
                let status = match current_norm {
                    Some(cur) if cur == path_norm => Cell::new("Used").fg(Color::Green),
                    _ => Cell::new("Installed"),
                };

                table.add_row(vec![
                    Cell::new(path_name).set_alignment(CellAlignment::Center),
                    status,
                ]);
            }

            output.write_line(&table.to_string())?;
        }
    }

    Ok(())
}

fn normalize_tag(tag: &str) -> &str {
    match tag.strip_prefix('v') {
        Some(rest) if rest.chars().next().map(|c| c.is_ascii_digit()).unwrap_or(false) => rest,
        _ => tag,
    }
}
