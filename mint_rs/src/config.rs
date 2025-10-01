use anyhow::Result;
use serde::{Deserialize, Serialize};
use std::fs;
use std::path::PathBuf;

#[derive(Debug, Serialize, Deserialize)]
pub struct Config {
    pub cache_dir: Option<PathBuf>,
    pub parallel_downloads: Option<usize>,
    pub timeout_seconds: Option<u64>,
    pub retry_attempts: Option<u32>,
    pub default_python: Option<String>,
    pub trusted_hosts: Vec<String>,
}

impl Default for Config {
    fn default() -> Self {
        Self {
            cache_dir: None,
            parallel_downloads: Some(num_cpus::get()),
            timeout_seconds: Some(30),
            retry_attempts: Some(3),
            default_python: None,
            trusted_hosts: vec!["pypi.org".to_string(), "files.pythonhosted.org".to_string()],
        }
    }
}

#[allow(dead_code)]
impl Config {
    pub fn load() -> Result<Self> {
        let config_path = Self::config_path()?;
        
        if config_path.exists() {
            let content = fs::read_to_string(&config_path)?;
            let config: Config = toml::from_str(&content)?;
            Ok(config)
        } else {
            let config = Config::default();
            config.save()?;
            Ok(config)
        }
    }

    pub fn save(&self) -> Result<()> {
        let config_path = Self::config_path()?;
        let content = toml::to_string_pretty(self)?;
        
        if let Some(parent) = config_path.parent() {
            fs::create_dir_all(parent)?;
        }
        
        fs::write(&config_path, content)?;
        Ok(())
    }

    fn config_path() -> Result<PathBuf> {
        let mut path = dirs::config_dir()
            .or_else(|| dirs::home_dir())
            .unwrap_or_else(|| PathBuf::from("."));
        path.push(".mint");
        path.push("config.toml");
        Ok(path)
    }
}
