mod filter;
mod reader;
mod args;
use anyhow::Result;
use std::sync::mpsc;
use std::thread;
use std::sync::{Arc, atomic::{AtomicBool, Ordering}};


// This file is part of RustLog, a simple CLI log filtering tool.
// It reads log files and filters lines based on a keyword.
fn main() -> Result<()> {
    env_logger::init(); // <--- initialize logging
    let args = args::parse_args();
    log::info!("Args: {:?}", args);
    if args.tail {
        // If tailing is enabled, start a thread to read the file continuously
        let (tx, rx) = mpsc::channel();
        let file_path = args.file_path.clone();

        // Set up a signal handler to gracefully shut down the tailing
        let running = Arc::new(AtomicBool::new(true));
        let r = running.clone();
        // Use ctrlc to handle Ctrl+C
        // This will allow us to stop the tailing thread when Ctrl+C is pressed
        log::info!("Press Ctrl+C to stop tailing...");
        ctrlc::set_handler(move || {
            r.store(false, Ordering::SeqCst);
            log::info!("Shutting down gracefully...");
        })?;
        let r = running.clone();

        // Spawn a thread to tail the file
        thread::spawn(move || {
            reader::tail_file(&file_path, tx, r).expect("Tailing failed");
        });

        // Main thread reads from the channel
        for line in rx {
            if line.contains(&args.keyword) {
                log::info!("Filtered: {}", line);
            }
        }
    } else {
        // Normal mode (non-tail)
        let lines = reader::read_lines(&args.file_path)?;
        let filtered = filter::filter_lines(lines, &args.keyword);
        for line in filtered {
            log::info!("Filtered: {}", line);
        }
    }

    Ok(())
}
