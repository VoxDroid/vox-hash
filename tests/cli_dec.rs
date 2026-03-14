use assert_cmd::Command;
use predicates::prelude::*;

#[test]
fn test_dec_sha1_brute_force() {
    let mut cmd = Command::cargo_bin("vox-hash").unwrap();
    cmd.arg("dec")
        .arg("--key")
        .arg("a94a8fe5ccb19ba61c4c0873d391e987982fbbd3") // "test"
        .arg("--algo")
        .arg("sha1")
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
fn test_dec_md5_wordlist() {
    let words_path = "tests/data/words.txt";
    let mut cmd = Command::cargo_bin("vox-hash").unwrap();
    cmd.arg("dec")
        .arg("--key")
        .arg("5f4dcc3b5aa765d61d8327deb882cf99") // "password" in MD5
        .arg("--wordlist")
        .arg(words_path)
        .arg("--auto")
        .arg("--noverbose");
    cmd.assert()
        .success()
        .stdout(predicate::str::contains("password"));
}

#[test]
fn test_dec_common_patterns() {
    let mut cmd = Command::cargo_bin("vox-hash").unwrap();
    cmd.arg("dec")
        .arg("--key")
        .arg("e10adc3949ba59abbe56e057f20f883e") // "123456" in MD5
        .arg("--auto")
        .arg("--common-patterns")
        .arg("true")
        .arg("--noverbose");
    cmd.assert()
        .success()
        .stdout(predicate::str::contains("123456"));
}

#[test]
fn test_dec_pattern_match() {
    let mut cmd = Command::cargo_bin("vox-hash").unwrap();
    cmd.arg("dec")
        .arg("--key")
        .arg("098f6bcd4621d373cade4e832627b4f6") // "test"
        .arg("--pattern")
        .arg("[a-z]{4}")
        .arg("--auto")
        .arg("--noverbose");
    cmd.assert()
        .success()
        .stdout(predicate::str::contains("test"));
}

#[test]
fn test_dec_no_match() {
    let mut cmd = Command::cargo_bin("vox-hash").unwrap();
    cmd.arg("dec")
        .arg("--key")
        .arg("098f6bcd4621d373cade4e832627b4f6") // "test"
        .arg("--min-len")
        .arg("1")
        .arg("--max-len")
        .arg("3")
        .arg("--auto")
        .arg("--noverbose");
    cmd.assert()
        .failure()
        .stdout(predicate::str::contains("No match found"));
}
