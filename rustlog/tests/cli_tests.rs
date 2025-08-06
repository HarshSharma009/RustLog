use assert_cmd::Command;
use std::fs::write;
use tempfile::NamedTempFile;

#[test]
fn shows_filtered_log_output() {
    // Create temp log file
    let file = NamedTempFile::new().expect("Failed to create temp file");
    let log_content = "INFO: start\nERROR: something went wrong\nWARN: disk almost full";
    write(file.path(), log_content).expect("Failed to write to temp file");

    // Run command on temp file
    let mut cmd = Command::cargo_bin("rustlog").unwrap();
    cmd.arg(file.path())
        .arg("ERROR")
        .env("RUST_LOG", "info") // <-- Add this line
        .assert()
        .success()
        .stderr(predicates::str::contains("ERROR: something went wrong"));
}
