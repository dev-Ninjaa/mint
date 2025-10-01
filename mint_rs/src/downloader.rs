use anyhow::Result;
use futures_util::StreamExt;
use indicatif::{ProgressBar, ProgressStyle};
use reqwest::Client;
use std::fs::OpenOptions;
use std::io::Write;
use std::time::Instant;
use crate::utils;

/// Download a single package async with streaming & progress
pub async fn download_package(client: &Client, url: &str, dest: &str) -> Result<()> {
    let start_time = Instant::now();
    let resp = client.get(url).send().await?;
    let total_size = resp.content_length().unwrap_or(0);

    // Enhanced progress bar with speed and ETA
    let style = ProgressStyle::default_bar()
        .template("{spinner:.green} [{bar:40.cyan/blue}] {bytes}/{total_bytes} ({percent}%) [{elapsed_precise}] {binary_bytes_per_sec} ETA: {eta}")?
        .progress_chars("#>-");

    let pb = ProgressBar::new(total_size);
    pb.set_style(style);

    let mut file = OpenOptions::new()
        .create(true)
        .write(true)
        .truncate(true)
        .open(dest)?;
    
    let mut stream = resp.bytes_stream();
    let mut downloaded = 0u64;

    while let Some(item) = stream.next().await {
        let chunk = item?;
        file.write_all(&chunk)?;
        downloaded += chunk.len() as u64;
        pb.inc(chunk.len() as u64);
    }
    
    let elapsed = start_time.elapsed();
    let speed = if elapsed.as_secs() > 0 {
        downloaded as f64 / elapsed.as_secs() as f64
    } else {
        0.0
    };
    
    pb.finish_with_message(format!(
        "✅ Downloaded {} ({}) in {:.2}s at {}/s", 
        dest, 
        utils::format_bytes(downloaded),
        elapsed.as_secs_f64(),
        utils::format_bytes(speed as u64)
    ));
    
    Ok(())
}
