use std::fs::File;
use std::io::{self, BufRead, BufReader, Seek, SeekFrom};
use std::path::Path;
use std::sync::{Arc, atomic::{AtomicBool, Ordering}};
use std::sync::mpsc::Sender;
use std::thread;
use std::time::Duration;
use anyhow::Result;

pub fn read_lines<P: AsRef<Path>>(file_path: P) -> Result<Vec<String>> {
    let file = File::open(file_path)?;
    let reader = io::BufReader::new(file);
    let lines = reader.lines().collect::<Result<_, _>>()?;
    Ok(lines)
}

pub fn tail_file<P: AsRef<Path>>(file_path: P, tx: Sender<String>, running: Arc<AtomicBool>) -> Result<()> {
    let mut file = File::open(file_path)?;
    file.seek(SeekFrom::End(0))?; // Move to the end of the file
    
    let mut reader = BufReader::new(file);
    
    while running.load(Ordering::SeqCst) {
        let mut line = String::new();
        let bytes_read = reader.read_line(&mut line)?;

        if bytes_read > 0 {
            tx.send(line.trim_end().to_string())?;
        } else {
            thread::sleep(Duration::from_millis(500));
        }
    }

    Ok(())
}
