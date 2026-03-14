use assert_cmd::Command;
use predicates::prelude::*;
use std::fs;

#[test]
fn test_enc_sha1() {
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
fn test_enc_md5() {
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
fn test_dec_wordlist() {
    let words_path = "tests/data/words.txt";
    let mut cmd = Command::cargo_bin("vox-hash").unwrap();
    cmd.arg("dec")
        .arg("--key")
        .arg("5baa61e4c9b93f3f0682250b6cf8331b7ee68fd8") // "password"
        .arg("--wordlist")
        .arg(words_path)
        .arg("--max-len")
        .arg("6")
        .arg("--noverbose");
    cmd.assert()
        .success()
        .stdout(predicate::str::contains("password"));
}

#[test]
fn test_dec_brute_force() {
    let mut cmd = Command::cargo_bin("vox-hash").unwrap();
    cmd.arg("dec")
        .arg("--key")
        .arg("098f6bcd4621d373cade4e832627b4f6") // "test"
        .arg("--algo")
        .arg("md5")
        .arg("--min-len")
        .arg("4")
        .arg("--max-len")
        .arg("4")
        .arg("--charset-type")
        .arg("lowercase")
        .arg("--noverbose");
    cmd.assert()
        .success()
        .stdout(predicate::str::contains("test"));
}

#[test]
fn test_bulk_enc() {
    let input_path = "tests/data/words.txt";
    let output_path = "tests/data/bulk_enc_output.txt";
    let mut cmd = Command::cargo_bin("vox-hash").unwrap();
    cmd.arg("bulk-enc")
        .arg("--algo")
        .arg("sha1")
        .arg("--input")
        .arg(input_path)
        .arg("--output")
        .arg(output_path)
        .arg("--noverbose");
    cmd.assert().success();

    let contents = fs::read_to_string(output_path).unwrap();
    assert!(contents.contains("5baa61e4c9b93f3f0682250b6cf8331b7ee68fd8"));
}

#[test]
fn test_generate_table_and_dec() {
    let table_path = "tests/data/table.json";
    let mut gen_cmd = Command::cargo_bin("vox-hash").unwrap();
    gen_cmd
        .arg("generate-table")
        .arg("--output")
        .arg(table_path)
        .arg("--min-len")
        .arg("1")
        .arg("--max-len")
        .arg("3")
        .arg("--algo")
        .arg("md5")
        .arg("--noverbose");
    gen_cmd.assert().success();

    let mut dec_cmd = Command::cargo_bin("vox-hash").unwrap();
    dec_cmd
        .arg("dec")
        .arg("--key")
        .arg("0cc175b9c0f1b6a831c399e269772661") // "a"
        .arg("--algo")
        .arg("md5")
        .arg("--rainbow-table")
        .arg(table_path)
        .arg("--noverbose");
    dec_cmd
        .assert()
        .success()
        .stdout(predicate::str::contains("a"));
}

#[test]
fn test_dec_pattern() {
    let mut cmd = Command::cargo_bin("vox-hash").unwrap();
    // SHA1 for "abc" is a9993e364706816aba3e25717850c26c9cd0d89d
    cmd.arg("dec")
        .arg("--key")
        .arg("a9993e364706816aba3e25717850c26c9cd0d89d")
        .arg("--pattern")
        .arg("[abc]{3}")
        .arg("--algo")
        .arg("sha1")
        .arg("--noverbose");
    cmd.assert()
        .success()
        .stdout(predicate::str::contains("abc"));
}
