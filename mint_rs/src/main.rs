mod cli;
mod resolver;
mod downloader;
mod installer;
mod cache;
mod config;
mod utils;
mod dependency;
mod requirements;
mod benchmark;

use tokio::runtime::Runtime;
use tracing::{error, info};

fn main() {
    // Initialize tracing for better logging
    tracing_subscriber::fmt::init();
    
    let rt = Runtime::new().expect("Failed to create Tokio runtime");
    rt.block_on(async {
        info!("Starting Mint package manager");
        if let Err(e) = cli::run().await {
            error!("mint_core error: {}", e);
            std::process::exit(1);
        }
        info!("Mint completed successfully");
    });
}
