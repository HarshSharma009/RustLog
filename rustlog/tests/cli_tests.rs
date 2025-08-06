use assert_cmd::Command;
use predicates::str::contains;

#[test]
fn shows_filtered_log_output() {
    let mut cmd = Command::cargo_bin("rustlog").unwrap();

    cmd.arg("./sample/sample.log")
        .arg("ERROR")
        .assert()
        .success()
        .stdout(contains("ERROR"));
}
