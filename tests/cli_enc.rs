use assert_cmd::Command;
use predicates::prelude::*;
use std::fs;

#[test]
fn test_enc_sha1_basic() {
    let mut cmd = Command::cargo_bin("vox-hash").unwrap();
    cmd.arg("enc")
        .arg("--algo")
        .arg("sha1")
        .arg("--str")
        .arg("test")
        .arg("--noverbose");
    cmd.assert().success().stdout(predicate::str::contains(
        "a94a8fe5ccb19ba61c4c0873d391e987982fbbd3",
    ));
}

#[test]
fn test_enc_md5_basic() {
    let mut cmd = Command::cargo_bin("vox-hash").unwrap();
    cmd.arg("enc")
        .arg("--algo")
        .arg("md5")
        .arg("--str")
        .arg("test")
        .arg("--noverbose");
    cmd.assert()
        .success()
        .stdout(predicate::str::contains("098f6bcd4621d373cade4e832627b4f6"));
}

#[test]
fn test_enc_json_output() {
    let mut cmd = Command::cargo_bin("vox-hash").unwrap();
    cmd.arg("enc")
        .arg("--algo")
        .arg("sha1")
        .arg("--str")
        .arg("test")
        .arg("--json")
        .arg("--noverbose");
    cmd.assert().success().stdout(predicate::str::contains(
        "\"result\":\"a94a8fe5ccb19ba61c4c0873d391e987982fbbd3\"",
    ));
}

#[test]
fn test_enc_file_output() {
    let output_path = "tests/data/enc_output.txt";
    let mut cmd = Command::cargo_bin("vox-hash").unwrap();
    cmd.arg("enc")
        .arg("--algo")
        .arg("sha1")
        .arg("--str")
        .arg("test")
        .arg("--output")
        .arg(output_path)
        .arg("--noverbose");
    cmd.assert().success();

    let contents = fs::read_to_string(output_path).unwrap();
    assert!(contents.contains("a94a8fe5ccb19ba61c4c0873d391e987982fbbd3"));
    fs::remove_file(output_path).unwrap();
}
