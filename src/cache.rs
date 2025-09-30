use anyhow::Result;
use std::fs;
use std::path::{Path, PathBuf};

pub fn cache_package(pkg_name: &str, filename: &str) -> Result<PathBuf> {
    let cache_dir = dirs::cache_dir().unwrap_or_else(|| PathBuf::from(".mint_cache"));
    fs::create_dir_all(&cache_dir)?;
    let dest = cache_dir.join(filename);
    fs::copy(filename, &dest)?;
    println!("✅ Cached {} at {:?}", pkg_name, dest);
    Ok(dest)
}
