use std::path::Path;

use anyhow::anyhow;
use anyhow::Result;
use futures_util::StreamExt;
use indicatif::ProgressBar;
use indicatif::ProgressStyle;
use reqwest::Client;
use tokio::io::AsyncWriteExt;

use crate::ports::Downloader;

pub struct ReqwestDownloader {
    client: Option<reqwest::Client>,
}

impl ReqwestDownloader {
    pub fn new(client: Option<&reqwest::Client>) -> Self {
        Self {
            client: client.cloned(),
        }
    }
}

impl Downloader for ReqwestDownloader {
    async fn download(&self, url: &str, dest: &Path) -> Result<()> {
        let client: &Client = self.client.as_ref().expect("Client not found");
        let response = client.get(url).send().await?.error_for_status()?;
        let total_size = response
            .content_length()
            .ok_or_else(|| anyhow!("Failed to get content length"))?;

        let pb = ProgressBar::new(total_size);
        pb.set_style(
            ProgressStyle::default_bar()
                .template("{bar:40.cyan/blue} {bytes}/{total_bytes} {msg}")
                .unwrap(),
        );

        let mut file = tokio::fs::File::create(dest).await?;
        let mut downloaded: u64 = 0;
        let mut stream = response.bytes_stream();

        while let Some(chunk) = stream.next().await {
            let chunk = chunk?;
            file.write_all(&chunk).await?;
            downloaded += chunk.len() as u64;
            pb.set_position(downloaded);
        }

        pb.finish_with_message(format!("Downloaded to {}", dest.display()));

        Ok(())
    }
}
