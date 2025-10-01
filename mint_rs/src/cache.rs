use anyhow::Result;
use std::fs;
use std::path::PathBuf;
use std::time::SystemTime;
use tracing::{info, debug};

/// Copy downloaded file into cache folder and return cached path
pub fn cache_package(pkg_name: &str, filename: &str) -> Result<PathBuf> {
    let mut cache_dir = dirs::cache_dir()
        .or_else(|| dirs::home_dir())
        .unwrap_or_else(|| PathBuf::from(".mint_cache"));
    cache_dir.push(".mint_cache");
    fs::create_dir_all(&cache_dir)?;
    
    let src = std::path::Path::new(filename);
    let dest = cache_dir.join(src.file_name().unwrap_or_else(|| std::ffi::OsStr::new("package")));
    
    // Check if file already exists and is recent (within 24 hours)
    let should_cache = if dest.exists() {
        if let Ok(metadata) = fs::metadata(&dest) {
            if let Ok(modified) = metadata.modified() {
                let now = SystemTime::now();
                if let Ok(duration) = now.duration_since(modified) {
                    duration.as_secs() > 86400 // 24 hours
                } else {
                    true
                }
            } else {
                true
            }
        } else {
            true
        }
    } else {
        true
    };
    
    if should_cache {
        debug!("Caching {} to {:?}", pkg_name, dest);
        fs::copy(src, &dest)?;
        info!("✅ Cached {} -> {:?}", pkg_name, dest);
    } else {
        debug!("Using existing cache for {}", pkg_name);
    }
    
    Ok(dest)
}

/// Clean old cache files (older than 7 days)
pub fn clean_cache() -> Result<()> {
    let mut cache_dir = dirs::cache_dir()
        .or_else(|| dirs::home_dir())
        .unwrap_or_else(|| PathBuf::from(".mint_cache"));
    cache_dir.push(".mint_cache");
    
    if !cache_dir.exists() {
        return Ok(());
    }
    
    let entries = fs::read_dir(&cache_dir)?;
    let now = SystemTime::now();
    let mut cleaned = 0;
    
    for entry in entries {
        let entry = entry?;
        if let Ok(metadata) = entry.metadata() {
            if let Ok(modified) = metadata.modified() {
                if let Ok(duration) = now.duration_since(modified) {
                    if duration.as_secs() > 604800 { // 7 days
                        fs::remove_file(entry.path())?;
                        cleaned += 1;
                    }
                }
            }
        }
    }
    
    info!("Cleaned {} old cache files", cleaned);
    Ok(())
}
