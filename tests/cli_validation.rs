use assert_cmd::Command;
use predicates::prelude::*;

#[test]
fn test_validation_invalid_hash_length() {
    let mut cmd = Command::cargo_bin("vox-hash").unwrap();
    cmd.arg("dec")
        .arg("--key")
        .arg("abc") // Too short
        .arg("--auto")
        .arg("--noverbose");
    cmd.assert()
        .failure()
        .stderr(predicate::str::contains("Invalid hash: abc"));
}

#[test]
fn test_validation_invalid_min_max_len() {
    let mut cmd = Command::cargo_bin("vox-hash").unwrap();
    cmd.arg("dec")
        .arg("--key")
        .arg("098f6bcd4621d373cade4e832627b4f6")
        .arg("--min-len")
        .arg("5")
        .arg("--max-len")
        .arg("4")
        .arg("--noverbose");
    cmd.assert().failure().stderr(predicate::str::contains(
        "--min-len (5) must be <= --max-len (4)",
    ));
}
