use anyhow::Result;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::fs;
use std::path::PathBuf;
use tracing::{info, warn};

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Dependency {
    pub name: String,
    pub version: String,
    pub source: String,
    pub dependencies: Vec<Dependency>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct LockFile {
    pub packages: HashMap<String, Dependency>,
    pub metadata: LockFileMetadata,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct LockFileMetadata {
    pub version: String,
    pub python_version: String,
    pub generated_at: String,
    pub mint_version: String,
}

#[allow(dead_code)]
impl LockFile {
    pub fn new(python_version: String) -> Self {
        Self {
            packages: HashMap::new(),
            metadata: LockFileMetadata {
                version: "1.0".to_string(),
                python_version,
                generated_at: chrono::Utc::now().to_rfc3339(),
                mint_version: env!("CARGO_PKG_VERSION").to_string(),
            },
        }
    }

    pub fn load(path: &PathBuf) -> Result<Self> {
        if path.exists() {
            let content = fs::read_to_string(path)?;
            let lockfile: LockFile = toml::from_str(&content)?;
            info!("Loaded lock file with {} packages", lockfile.packages.len());
            Ok(lockfile)
        } else {
            warn!("Lock file not found, creating new one");
            Ok(LockFile::new("unknown".to_string()))
        }
    }

    pub fn save(&self, path: &PathBuf) -> Result<()> {
        let content = toml::to_string_pretty(self)?;
        fs::write(path, content)?;
        info!("Saved lock file with {} packages", self.packages.len());
        Ok(())
    }

    pub fn add_package(&mut self, dep: Dependency) {
        self.packages.insert(dep.name.clone(), dep);
    }

    pub fn has_package(&self, name: &str) -> bool {
        self.packages.contains_key(name)
    }

    pub fn get_package(&self, name: &str) -> Option<&Dependency> {
        self.packages.get(name)
    }
}

/// Resolve package dependencies recursively
#[allow(dead_code)]
pub async fn resolve_dependencies(
    package_name: &str,
    version: Option<&str>,
    client: &reqwest::Client,
) -> Result<Vec<Dependency>> {
    let mut resolved = Vec::new();
    let mut visited = std::collections::HashSet::new();
    
    resolve_recursive(package_name, version, client, &mut resolved, &mut visited).await?;
    
    Ok(resolved)
}

#[allow(dead_code)]
async fn resolve_recursive(
    package_name: &str,
    version: Option<&str>,
    client: &reqwest::Client,
    resolved: &mut Vec<Dependency>,
    visited: &mut std::collections::HashSet<String>,
) -> Result<()> {
    if visited.contains(package_name) {
        return Ok(());
    }
    
    visited.insert(package_name.to_string());
    
    // Fetch package metadata
    let url = format!("https://pypi.org/pypi/{}/json", package_name);
    let resp = client.get(&url).send().await?;
    
    if !resp.status().is_success() {
        warn!("Package {} not found on PyPI", package_name);
        return Ok(());
    }
    
    let metadata: serde_json::Value = resp.json().await?;
    
    // Get version info
    let version_info = if let Some(v) = version {
        metadata["releases"][v].as_array()
    } else {
        metadata["releases"]
            .as_object()
            .and_then(|releases| releases.values().last())
            .and_then(|v| v.as_array())
    };
    
    if let Some(files) = version_info {
        if let Some(file) = files.first() {
            let version_str = version.unwrap_or("latest").to_string();
            let source_url = file["url"].as_str().unwrap_or("").to_string();
            
            let dep = Dependency {
                name: package_name.to_string(),
                version: version_str,
                source: source_url,
                dependencies: Vec::new(), // TODO: Parse dependencies from package metadata
            };
            
            resolved.push(dep);
        }
    }
    
    Ok(())
}
