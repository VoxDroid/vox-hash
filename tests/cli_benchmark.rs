use assert_cmd::Command;
use predicates::prelude::*;

#[test]
fn test_benchmark_basic() {
    let mut cmd = Command::cargo_bin("vox-hash").unwrap();
    cmd.arg("benchmark")
        .arg("--algo")
        .arg("sha1")
        .arg("--iterations")
        .arg("1000")
        .arg("--noverbose");
    cmd.assert()
        .success()
        .stdout(predicate::str::contains("hashes/second"));
}
