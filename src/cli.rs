use clap::{Parser, Subcommand};
use crate::{resolver, downloader, installer, cache};
use reqwest::Client;
use tokio::task;
use anyhow::Result;
use futures::stream::{FuturesUnordered, StreamExt};

#[derive(Parser)]
#[command(name = "mint")]
#[command(about = "Ultra-fast Python package manager", long_about = None)]
pub struct Cli {
    #[command(subcommand)]
    pub command: Commands,
}

#[derive(Subcommand)]
pub enum Commands {
    PipInstall { packages: Vec<String> },
    PipUninstall { packages: Vec<String> },
    VenvCreate { name: String },
    Run { venv: String, script: String },
}

pub async fn run() -> Result<()> {
    let cli = Cli::parse();
    let client = Client::new();

    match cli.command {
        Commands::PipInstall { packages } => {
            let mut futures = FuturesUnordered::new();

            for pkg in packages {
                let c = client.clone();
                let pkg_clone = pkg.clone();

                futures.push(task::spawn(async move {
                    let meta = resolver::fetch_package_metadata(&c, &pkg_clone).await.unwrap();

                    if let Some(releases) = meta.releases() {
                        if let Some(versions) = releases.values().last() {
                            if let Some(file) = versions[0].as_object() {
                                if let Some(url_str) = file.get("url").and_then(|u| u.as_str()) {
                                    let filename = url_str.split('/').last().unwrap();
                                    downloader::download_package(&c, url_str, filename).await.unwrap();
                                    let cached = cache::cache_package(&pkg_clone, filename).unwrap();
                                    installer::install_wheel(cached.to_str().unwrap()).unwrap();
                                }
                            }
                        }
                    }
                }));
            }

            while let Some(_) = futures.next().await {}
        }
        Commands::PipUninstall { packages } => {
            for pkg in packages {
                installer::uninstall_package(&pkg)?;
            }
        }
        Commands::VenvCreate { name } => {
            installer::create_venv(&name)?;
        }
        Commands::Run { venv, script } => {
            installer::run_in_venv(&venv, &script)?;
        }
    }

    Ok(())
}
