use std::fs;

use anyhow::anyhow;
use anyhow::Result;
use liblzma::read::XzDecoder;
use zip::ZipArchive as ZipReader;

use crate::domain::package::Package;
use crate::domain::version::LocalVersion;
use crate::ports::Archive;

pub struct LocalArchive;

impl Archive for LocalArchive {
    async fn extract(&self, package: Package, file: LocalVersion) -> anyhow::Result<()> {
        unarchive(package, file).await
    }
}

async fn unarchive(package: Package, file: LocalVersion) -> Result<()> {
    let path = format!("{}/{}.{}", file.path, file.file_name, file.file_format);
    tokio::task::spawn_blocking(move || expand(package, file))
        .await?
        .map_err(|e| anyhow!(e))?;

    tokio::fs::remove_file(path).await?;

    Ok(())
}

fn expand(package: Package, tmp: LocalVersion) -> Result<()> {
    use std::fs::File;
    use std::os::unix::fs::PermissionsExt;

    use anyhow::Context;
    use flate2::read::GzDecoder;
    use tar::Archive;

    if fs::metadata(&tmp.file_name).is_ok() {
        fs::remove_dir_all(&tmp.file_name)?;
    }

    let file_path = format!("{}/{}.{}", tmp.path, tmp.file_name, tmp.file_format);
    let file = File::open(&file_path).map_err(|error| {
        anyhow!(
            "Failed to open file {}.{}, file doesn't exist. additional info: {error}",
            tmp.file_name,
            tmp.file_format,
        )
    })?;

    let output = format!("{}/{}", tmp.path, tmp.file_name);

    let context_msg = format!(
        "Failed to decompress or extract file {}.{}",
        tmp.file_name, tmp.file_format
    );

    match tmp.file_format.as_str() {
        "tar.gz" => {
            let decompress_stream = GzDecoder::new(file);
            let mut archive = Archive::new(decompress_stream);
            archive.unpack(&output).with_context(|| context_msg)?;
        }
        "tar.xz" => {
            let decompress_stream = XzDecoder::new(file);
            let mut archive = Archive::new(decompress_stream);
            archive.unpack(&output).with_context(|| context_msg)?;
        }
        "zip" => {
            let mut archive = ZipReader::new(file)
                .map_err(|error| anyhow!("{context_msg}. additional info: {error}"))?;
            archive.extract(&output).with_context(|| context_msg)?;
        }
        _ => return Err(anyhow!("Unsupported file format")),
    }

    let binary = &format!(
        "{}/{}/{}",
        tmp.file_name,
        package.binary_path(),
        package.binary_name()
    );

    let mut perms = fs::metadata(binary)?.permissions();
    perms.set_mode(0o551);
    fs::set_permissions(binary, perms)?;

    Ok(())
}
