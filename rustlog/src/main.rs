mod filter;
mod reader;
mod reader_async;
mod args;

use anyhow::Result;
use std::sync::{Arc, atomic::{AtomicBool, Ordering}};
use tokio::sync::mpsc;
use tokio::signal;

// This file is part of RustLog, a simple CLI log filtering tool.
// It reads log files and filters lines based on a keyword.
#[tokio::main]
async fn main() -> Result<()> {
    env_logger::init();
    let args = args::parse_args();
    log::info!("Starting RustLog with args: {:?}", args);

    if args.tail {
        log::info!("Tail mode activated. Monitoring file: {:?}", args.file_path);
        let (tx, mut rx) = mpsc::channel(100);
        let file_path = args.file_path.clone();
        let keyword = args.keyword.clone();

        let running = Arc::new(AtomicBool::new(true));
        let r = running.clone();

        // Handle Ctrl+C for graceful shutdown
        tokio::spawn(async move {
            if let Err(e) = signal::ctrl_c().await {
                log::error!("Failed to install Ctrl+C handler: {}", e);
                return;
            }
            log::warn!("Received Ctrl+C. Initiating shutdown...");
            r.store(false, Ordering::SeqCst);
        });

        // Spawn async tailing task
        let r = running.clone();
        tokio::spawn(async move {
            log::debug!("Starting tailing task...");
            if let Err(e) = reader_async::tail_file_async(file_path, tx, r).await {
                log::error!("Tailing failed: {}", e);
            }
        });

        // Main async loop reading from the channel
        while let Some(line) = rx.recv().await {
            if line.contains(&keyword) {
                log::info!("Filtered: {}", line);
            } else {
                log::debug!("Ignored: {}", line);
            }
        }
        log::info!("Tailing ended. Exiting.");
    } else {
        log::info!("Non-tail mode. Reading full file: {:?}", args.file_path);
        match reader::read_lines(&args.file_path) {
            Ok(lines) => {
                let filtered = filter::filter_lines(lines, &args.keyword);
                for line in filtered {
                    log::info!("Filtered: {}", line);
                }
            },
            Err(e) => {
                log::error!("Failed to read file: {}", e);
            }
        }
    }

    log::info!("RustLog finished execution.");
    Ok(())
}
