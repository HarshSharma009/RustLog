mod filter;
mod reader;
mod reader_async;
mod args;

use anyhow::Result;
use std::sync::{Arc, atomic::{AtomicBool, Ordering}};
use tokio::sync::mpsc;
use tokio::signal;
use tracing_subscriber::{EnvFilter, fmt};

fn init_tracing() {
    let subscriber = fmt()
        .with_env_filter(EnvFilter::from_default_env())
        .with_target(false)
        .with_level(true)
        .with_thread_names(true)
        .with_file(true)
        .with_line_number(true)
        .compact() // Use .json() here for structured logs in production
        .finish();

    tracing::subscriber::set_global_default(subscriber).expect("setting default subscriber failed");
}

#[tokio::main]
async fn main() -> Result<()> {
    init_tracing();
    let args = args::parse_args();
    tracing::info!(?args, "Starting RustLog");

    if args.tail {
        tracing::info!(file = %std::path::Path::new(&args.file_path).display(), "Tail mode activated");
        let (tx, mut rx) = mpsc::channel(100);
        let file_path = args.file_path.clone();
        let keyword = args.keyword.clone();

        let running = Arc::new(AtomicBool::new(true));
        let r = running.clone();

        tokio::spawn(async move {
            signal::ctrl_c().await.expect("Failed to install Ctrl+C handler");
            tracing::info!("Shutting down gracefully...");
            r.store(false, Ordering::SeqCst);
        });

        let r = running.clone();
        tokio::spawn(async move {
            if let Err(e) = reader_async::tail_file_async(file_path, tx, r).await {
                tracing::error!(error = %e, "Tailing failed");
            }
        });

        while let Some(line) = rx.recv().await {
            if line.contains(&keyword) {
                tracing::info!(target: "filtered", %line);
            }
        }
    } else {
        tracing::info!(file = %std::path::Path::new(&args.file_path).display(), "Non-tail mode: Reading file");
        match reader::read_lines(&args.file_path) {
            Ok(lines) => {
                let filtered = filter::filter_lines(lines, &args.keyword);
                for line in filtered {
                    tracing::info!(target: "filtered", %line);
                }
            },
            Err(e) => {
                tracing::error!(error = %e, "Failed to read file");
            }
        }
    }

    Ok(())
}
