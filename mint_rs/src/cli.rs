use clap::{Parser, Subcommand};
use crate::{resolver, downloader, installer, cache};
use reqwest::Client;
use tokio::task;
use anyhow::Result;
use futures_util::stream::{FuturesUnordered, StreamExt};
use std::sync::Arc;
use tracing::{info, error, warn};
use std::fs;

/// Install a single package with retry logic and proper error handling
async fn install_package(
    client: &Arc<Client>,
    package: &str,
    venv_path: Option<&str>,
    _force: bool,
) -> Result<()> {
    info!("Installing package: {}", package);
    
    // Parse package name and version
    let (pkg_name, version) = parse_package_spec(package);
    
    // Fetch metadata
    let meta = resolver::fetch_package_metadata(client, &pkg_name).await?;
    
    // Get download URLs
    let urls = get_download_urls(&meta, version.as_deref())?;
    
    // Download and install
    for (url, filename) in urls {
        // Retry download up to 3 times
        for attempt in 1..=3 {
            if let Ok(_) = downloader::download_package(client, &url, &filename).await {
                break;
            } else if attempt == 3 {
                anyhow::bail!("Failed to download {} after 3 attempts", filename);
            } else {
                warn!("Download attempt {}/3 failed for {}", attempt, filename);
            }
        }
        
        // Cache and install
        let cached_path = cache::cache_package(&pkg_name, &filename)?;
        installer::install_wheel(
            cached_path.to_str().unwrap_or(""),
            venv_path,
        )?;
    }
    
    info!("Successfully installed {}", package);
    Ok(())
}

/// Parse package specification (name==version, name>=1.0.0, etc.)
fn parse_package_spec(spec: &str) -> (String, Option<String>) {
    if let Some(pos) = spec.find("==") {
        (spec[..pos].to_string(), Some(spec[pos + 2..].to_string()))
    } else if let Some(pos) = spec.find(">=") {
        (spec[..pos].to_string(), Some(format!(">={}", &spec[pos + 2..])))
    } else if let Some(pos) = spec.find("<=") {
        (spec[..pos].to_string(), Some(format!("<={}", &spec[pos + 2..])))
    } else if let Some(pos) = spec.find(">") {
        (spec[..pos].to_string(), Some(format!(">{}", &spec[pos + 1..])))
    } else if let Some(pos) = spec.find("<") {
        (spec[..pos].to_string(), Some(format!("<{}", &spec[pos + 1..])))
    } else {
        (spec.to_string(), None)
    }
}

/// Extract download URLs from package metadata
fn get_download_urls(meta: &resolver::PyPiResponse, version: Option<&str>) -> Result<Vec<(String, String)>> {
    let mut urls = Vec::new();
    
    if let Some(releases) = meta.releases() {
        // If version specified, use that; otherwise use latest
        let target_release = if let Some(v) = version {
            releases.get(v).and_then(|r| r.as_array())
        } else {
            releases.values().last().and_then(|r| r.as_array())
        };
        
        if let Some(files) = target_release {
            for file in files {
                if let Some(url) = file.get("url").and_then(|u| u.as_str()) {
                    let filename = url.split('/').last().unwrap_or("package.whl").to_string();
                    urls.push((url.to_string(), filename));
                }
            }
        }
    }
    
    if urls.is_empty() {
        anyhow::bail!("No download URLs found for package");
    }
    
    Ok(urls)
}

#[derive(Parser)]
#[command(name = "mint")]
#[command(about = "Ultra-fast Python package manager", long_about = None)]
pub struct Cli {
    #[command(subcommand)]
    pub command: Commands,
}

#[derive(Subcommand)]
pub enum Commands {
    /// Install packages (supports package==version, >=1.0.0, etc.)
    Install {
        packages: Vec<String>,
        /// Virtual environment path
        #[arg(short = 'v', long = "venv")]
        venv: Option<String>,
        /// Force reinstall packages
        #[arg(short = 'f', long = "force")]
        force: bool,
        /// Install development dependencies
        #[arg(short = 'd', long = "dev")]
        dev: bool,
        /// Number of parallel downloads
        #[arg(short = 'j', long = "jobs")]
        jobs: Option<usize>,
    },
    /// Uninstall packages
    Uninstall {
        packages: Vec<String>,
        /// Virtual environment path
        #[arg(short = 'v', long = "venv")]
        venv: Option<String>,
        /// Confirm uninstallation
        #[arg(short = 'y', long = "yes")]
        yes: bool,
    },
    /// Create a virtual environment
    VenvCreate { 
        name: String,
        /// Python version to use
        #[arg(short = 'p', long = "python")]
        python: Option<String>,
    },
    /// Delete a virtual environment
    VenvDelete { 
        name: String,
        /// Force deletion without confirmation
        #[arg(short = 'f', long = "force")]
        force: bool,
    },
    /// Run a Python script in virtual environment
    Run { 
        venv: String, 
        script: String,
        /// Pass arguments to the script
        #[arg(short = 'a', long = "args")]
        args: Vec<String>,
    },
    /// List installed packages
    List {
        /// Virtual environment path
        #[arg(short = 'v', long = "venv")]
        venv: Option<String>,
        /// Show outdated packages
        #[arg(short = 'o', long = "outdated")]
        outdated: bool,
    },
    /// Show package information
    Show {
        package: String,
        /// Virtual environment path
        #[arg(short = 'v', long = "venv")]
        venv: Option<String>,
    },
    /// Search for packages
    Search {
        query: String,
        /// Limit results
        #[arg(short = 'l', long = "limit")]
        limit: Option<usize>,
    },
    /// Clean old cache files
    CacheClean,
    /// Show cache information
    CacheInfo,
    /// Install from requirements.txt file
    InstallRequirements {
        /// Path to requirements.txt file
        #[arg(short = 'r', long = "requirements")]
        requirements: Option<String>,
        /// Virtual environment path
        #[arg(short = 'v', long = "venv")]
        venv: Option<String>,
    },
    /// Generate requirements.txt from installed packages
    Freeze {
        /// Output file path
        #[arg(short = 'o', long = "output")]
        output: Option<String>,
        /// Virtual environment path
        #[arg(short = 'v', long = "venv")]
        venv: Option<String>,
    },
}

pub async fn run() -> Result<()> {
    let cli = Cli::parse();
    let client = Arc::new(Client::new());

    match cli.command {
        Commands::Install { packages, venv, force, dev: _dev, jobs } => {
            info!("Installing packages: {:?}", packages);
            let max_jobs = jobs.unwrap_or_else(|| num_cpus::get());
            info!("Using {} parallel jobs", max_jobs);
            
            let packages_count = packages.len();
            let mut top = FuturesUnordered::new();
            for pkg in &packages {
                let c = Arc::clone(&client);
                let pkg_clone = pkg.clone();
                let venv_clone = venv.clone();
                top.push(task::spawn(async move {
                    install_package(&c, &pkg_clone, venv_clone.as_deref(), force).await
                }));
            }
            
            let mut completed = 0;
            while let Some(result) = top.next().await {
                match result {
                    Ok(Ok(_)) => {
                        completed += 1;
                        info!("Completed installation {}/{}", completed, packages_count);
                    }
                    Ok(Err(e)) => error!("Installation failed: {}", e),
                    Err(e) => error!("Task failed: {}", e),
                }
            }
        }

        Commands::Uninstall { packages, venv, yes } => {
            for pkg in packages {
                if !yes {
                    println!("Are you sure you want to uninstall {}? (y/N)", pkg);
                    let mut input = String::new();
                    std::io::stdin().read_line(&mut input)?;
                    if !input.trim().to_lowercase().starts_with('y') {
                        info!("Skipping uninstall of {}", pkg);
                        continue;
                    }
                }
                
                let v = venv.as_deref();
                if let Err(e) = installer::uninstall_package(&pkg, v) {
                    error!("Failed to uninstall {}: {}", pkg, e);
                } else {
                    info!("Successfully uninstalled {}", pkg);
                }
            }
        }

        Commands::VenvCreate { name, python: _python } => {
            info!("Creating virtual environment: {}", name);
            installer::create_venv(&name)?;
            info!("Successfully created virtual environment: {}", name);
        }

        Commands::VenvDelete { name, force } => {
            if !force {
                println!("Are you sure you want to delete virtual environment {}? (y/N)", name);
                let mut input = String::new();
                std::io::stdin().read_line(&mut input)?;
                if !input.trim().to_lowercase().starts_with('y') {
                    info!("Skipping deletion of {}", name);
                    return Ok(());
                }
            }
            
            std::fs::remove_dir_all(&name)?;
            info!("Successfully deleted virtual environment: {}", name);
        }

        Commands::Run { venv, script, args } => {
            let full_script = if args.is_empty() {
                script
            } else {
                format!("{} {}", script, args.join(" "))
            };
            installer::run_in_venv(&venv, &full_script)?;
        }

        Commands::List { venv: _venv, outdated: _outdated } => {
            info!("Listing packages in virtual environment");
            // TODO: Implement package listing
            println!("Package listing not yet implemented");
        }

        Commands::Show { package, venv: _venv } => {
            info!("Showing information for package: {}", package);
            // TODO: Implement package information display
            println!("Package information display not yet implemented");
        }

        Commands::Search { query, limit: _limit } => {
            info!("Searching for packages: {}", query);
            // TODO: Implement package search
            println!("Package search not yet implemented");
        }

        Commands::CacheClean => {
            info!("Cleaning old cache files");
            cache::clean_cache()?;
            info!("Cache cleanup completed");
        }

        Commands::CacheInfo => {
            info!("Showing cache information");
            // TODO: Implement cache info display
            println!("Cache information display not yet implemented");
        }

        Commands::InstallRequirements { requirements, venv } => {
            let req_path = requirements.unwrap_or_else(|| "requirements.txt".to_string());
            let path = std::path::PathBuf::from(&req_path);
            
            match crate::requirements::parse_requirements(&path) {
                Ok(packages) => {
                    info!("Installing {} packages from requirements file", packages.len());
                    
                    let _max_jobs = num_cpus::get();
                    let mut top = FuturesUnordered::new();
                    
                    for pkg in &packages {
                        let c = Arc::clone(&client);
                        let pkg_clone = pkg.clone();
                        let venv_clone = venv.clone();
                        top.push(task::spawn(async move {
                            install_package(&c, &pkg_clone, venv_clone.as_deref(), false).await
                        }));
                    }
                    
                    let mut completed = 0;
                    while let Some(result) = top.next().await {
                        match result {
                            Ok(Ok(_)) => {
                                completed += 1;
                                info!("Completed installation {}/{}", completed, packages.len());
                            }
                            Ok(Err(e)) => error!("Installation failed: {}", e),
                            Err(e) => error!("Task failed: {}", e),
                        }
                    }
                }
                Err(e) => error!("Failed to parse requirements file: {}", e),
            }
        }

        Commands::Freeze { output, venv } => {
            match crate::requirements::generate_requirements(venv.as_deref()) {
                Ok(requirements) => {
                    if let Some(output_path) = output {
                        let path = std::path::PathBuf::from(output_path);
                        fs::write(&path, requirements)?;
                        info!("Requirements saved to {:?}", path);
                    } else {
                        print!("{}", requirements);
                    }
                }
                Err(e) => error!("Failed to generate requirements: {}", e),
            }
        }
    }

    Ok(())
}
