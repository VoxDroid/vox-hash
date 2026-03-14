use assert_cmd::Command;
use predicates::prelude::*;
use std::fs;

#[test]
fn test_bulk_dec_plain() {
    let input_path = "tests/data/hashes.txt";
    let output_path = "tests/data/bulk_dec_plain.txt";
    let mut cmd = Command::cargo_bin("vox-hash").unwrap();
    cmd.arg("bulk-dec")
        .arg("--input")
        .arg(input_path)
        .arg("--output")
        .arg(output_path)
        .arg("--auto")
        .arg("--common-patterns")
        .arg("true")
        .arg("--max-len")
        .arg("1")
        .arg("--noverbose");
    cmd.assert().success();

    let contents = fs::read_to_string(output_path).unwrap();
    // "password" in SHA1 is there. Empty string in MD5 is not matched because min_len default is 1.
    assert!(contents.contains("password"));
    fs::remove_file(output_path).unwrap();
}

#[test]
fn test_bulk_dec_only_success() {
    let input_path = "tests/data/hashes.txt";
    let mut cmd = Command::cargo_bin("vox-hash").unwrap();
    cmd.arg("bulk-dec")
        .arg("--input")
        .arg(input_path)
        .arg("--auto")
        .arg("--only-success")
        .arg("--max-len")
        .arg("1")
        .arg("--noverbose");
    cmd.assert()
        .success()
        .stdout(predicate::str::contains("password"))
        .stdout(predicate::str::contains("No match found").not());
}

#[test]
fn test_bulk_dec_json() {
    let input_path = "tests/data/hashes.txt";
    let output_path = "tests/data/bulk_dec.json";
    let mut cmd = Command::cargo_bin("vox-hash").unwrap();
    cmd.arg("bulk-dec")
        .arg("--input")
        .arg(input_path)
        .arg("--output")
        .arg(output_path)
        .arg("--json")
        .arg("--auto")
        .arg("--common-patterns")
        .arg("true")
        .arg("--max-len")
        .arg("1")
        .arg("--noverbose");
    cmd.assert().success();

    let contents = fs::read_to_string(output_path).unwrap();
    assert!(contents.contains("\"result\":\"password\""));
    fs::remove_file(output_path).unwrap();
}
