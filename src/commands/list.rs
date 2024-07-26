use std::fs;
use std::path::PathBuf;

use anyhow::anyhow;
use anyhow::Error;
use anyhow::Result;
use comfy_table::modifiers::UTF8_ROUND_CORNERS;
use comfy_table::presets::UTF8_FULL;
use comfy_table::Cell;
use comfy_table::CellAlignment;
use comfy_table::Color;
use comfy_table::Table;
use regex::Regex;

use super::install::Package;
use crate::fs::get_downloads_directory;
use crate::helpers::version::is_version_used;

pub async fn list(package: Package) -> Result<(), Error> {
    let downloads_dir = get_downloads_directory(package.clone()).await?;

    let paths: Vec<PathBuf> = fs::read_dir(downloads_dir)?
        .filter_map(Result::ok)
        .map(|entry| entry.path())
        .collect();

    if paths.is_empty() {
        return Err(anyhow!("There are no versions installed"));
    }

    let mut table = Table::new();
    let header = vec!["Version", "Status"];
    table.load_preset(UTF8_FULL).apply_modifier(UTF8_ROUND_CORNERS);
    table.set_header(header);

    for path in paths {
        if !path.is_dir() {
            continue;
        }

        let path_name = path.file_name().unwrap().to_str().unwrap();

        if !is_version(path_name) {
            continue;
        }

        let status = if is_version_used(path_name, package.clone()).await {
            Cell::new("Used").fg(Color::Green)
        } else {
            Cell::new("Installed")
        };

        table.add_row(vec![
            Cell::new(path_name).set_alignment(CellAlignment::Center),
            status,
        ]);
    }

    println!("{table}");

    Ok(())
}

fn is_version(name: &str) -> bool {
    match name {
        "stable" => true,
        nightly_name if nightly_name.contains("nightly") => true,
        name => {
            let version_regex = Regex::new(r"^v?[0-9]+\.[0-9]+\.[0-9]+$").unwrap();
            let hash_regex = Regex::new(r"\b[0-9a-f]{5,40}\b").unwrap();

            if version_regex.is_match(name) {
                return true;
            }

            hash_regex.is_match(name)
        }
    }
}
