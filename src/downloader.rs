use anyhow::Result;
use futures_util::StreamExt;
use indicatif::{ProgressBar, ProgressStyle};
use reqwest::Client;
use std::fs::File;
use std::io::Write;

pub async fn download_package(client: &Client, url: &str, dest: &str) -> Result<()> {
    // Make request
    let resp = client.get(url).send().await?;
    let total_size = resp
        .content_length()
        .ok_or_else(|| anyhow::anyhow!("Failed to get content length"))?;

    // Progress bar
    let pb = ProgressBar::new(total_size);
    let style = ProgressStyle::default_bar()
        .template("{spinner:.green} [{bar:40.cyan/blue}] {bytes}/{total_bytes} ({eta})")?
        .progress_chars("#>-");
    pb.set_style(style);

    // Stream bytes
    let mut stream = resp.bytes_stream();

    // Open file
    let mut file = File::create(dest)?;

    while let Some(item) = stream.next().await {
        let chunk = item?;
        file.write_all(&chunk)?; // Use &chunk to fix `[u8]` size error
        pb.inc(chunk.len() as u64);
    }

    pb.finish_with_message(format!("Downloaded {}", dest));
    Ok(())
}
