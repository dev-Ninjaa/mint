mod cli;
mod resolver;
mod downloader;
mod installer;
mod cache;

use tokio::runtime::Runtime;

fn main() {
    let rt = Runtime::new().expect("Failed to create Tokio runtime");
    rt.block_on(async {
        if let Err(e) = cli::run().await {
            eprintln!("mint_core error: {}", e);
            std::process::exit(1);
        }
    });
}
