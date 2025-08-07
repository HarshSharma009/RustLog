use std::path::Path;
use std::sync::{Arc, atomic::{AtomicBool, Ordering}};
use tokio::fs::File;
use tokio::io::{self, AsyncBufReadExt, BufReader};
use tokio::sync::mpsc::Sender;
use anyhow::Result;
use tokio::time::{sleep, Duration};

pub async fn tail_file_async<P: AsRef<Path>>(file_path: P, tx: Sender<String>, running: Arc<AtomicBool>) -> Result<()> {
    let file = File::open(file_path).await?;
    let mut reader = BufReader::new(file);

    let mut buffer = String::new();

    loop {
        if !running.load(Ordering::SeqCst) {
            break;
        }

        buffer.clear();
        let bytes_read = reader.read_line(&mut buffer).await?;

        if bytes_read > 0 {
            tx.send(buffer.trim_end().to_string()).await.unwrap_or_else(|e| {
                log::error!("Failed to send line: {}", e);
            });
        } else {
            sleep(Duration::from_millis(500)).await;
        }
    }

    Ok(())
}
