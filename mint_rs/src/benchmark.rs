use anyhow::Result;
use std::time::Instant;
use tracing::info;
use futures_util::StreamExt;

/// Benchmark download performance
#[allow(dead_code)]
pub async fn benchmark_download(client: &reqwest::Client, url: &str, iterations: usize) -> Result<()> {
    info!("Starting download benchmark with {} iterations", iterations);
    
    let mut times = Vec::new();
    let mut sizes = Vec::new();
    
    for i in 0..iterations {
        let start = Instant::now();
        
        let resp = client.get(url).send().await?;
        let _total_size = resp.content_length().unwrap_or(0);
        
        let mut downloaded = 0u64;
        let mut stream = resp.bytes_stream();
        
        while let Some(chunk) = stream.next().await {
            let chunk = chunk?;
            downloaded += chunk.len() as u64;
        }
        
        let elapsed = start.elapsed();
        times.push(elapsed.as_secs_f64());
        sizes.push(downloaded);
        
        info!("Iteration {}: {:.2}s, {} bytes", i + 1, elapsed.as_secs_f64(), downloaded);
    }
    
    let avg_time = times.iter().sum::<f64>() / times.len() as f64;
    let avg_size = sizes.iter().sum::<u64>() / sizes.len() as u64;
    let avg_speed = avg_size as f64 / avg_time;
    
    info!("Benchmark Results:");
    info!("  Average time: {:.2}s", avg_time);
    info!("  Average size: {} bytes", avg_size);
    info!("  Average speed: {:.2} bytes/s ({:.2} MB/s)", avg_speed, avg_speed / 1_000_000.0);
    
    Ok(())
}

/// Performance metrics for package operations
#[allow(dead_code)]
pub struct PerformanceMetrics {
    pub download_time: f64,
    pub install_time: f64,
    pub total_time: f64,
    pub package_size: u64,
    pub download_speed: f64,
}

#[allow(dead_code)]
impl PerformanceMetrics {
    pub fn new() -> Self {
        Self {
            download_time: 0.0,
            install_time: 0.0,
            total_time: 0.0,
            package_size: 0,
            download_speed: 0.0,
        }
    }
    
    pub fn log_summary(&self) {
        info!("Performance Summary:");
        info!("  Download time: {:.2}s", self.download_time);
        info!("  Install time: {:.2}s", self.install_time);
        info!("  Total time: {:.2}s", self.total_time);
        info!("  Package size: {} bytes", self.package_size);
        info!("  Download speed: {:.2} MB/s", self.download_speed / 1_000_000.0);
    }
}
