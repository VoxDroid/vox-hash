use assert_cmd::Command;
use std::fs;

#[test]
fn test_bulk_enc_plain() {
    let input_path = "tests/data/words.txt";
    let output_path = "tests/data/bulk_enc_plain.txt";
    let mut cmd = Command::cargo_bin("vox-hash").unwrap();
    cmd.arg("bulk-enc")
        .arg("--input")
        .arg(input_path)
        .arg("--output")
        .arg(output_path)
        .arg("--algo")
        .arg("sha1")
        .arg("--noverbose");
    cmd.assert().success();

    let contents = fs::read_to_string(output_path).unwrap();
    assert!(contents.contains("5baa61e4c9b93f3f0682250b6cf8331b7ee68fd8")); // "password"
    fs::remove_file(output_path).unwrap();
}

#[test]
fn test_bulk_enc_json() {
    let input_path = "tests/data/words.txt";
    let output_path = "tests/data/bulk_enc.json";
    let mut cmd = Command::cargo_bin("vox-hash").unwrap();
    cmd.arg("bulk-enc")
        .arg("--input")
        .arg(input_path)
        .arg("--output")
        .arg(output_path)
        .arg("--json")
        .arg("--algo")
        .arg("md5")
        .arg("--noverbose");
    cmd.assert().success();

    let contents = fs::read_to_string(output_path).unwrap();
    assert!(contents.contains("\"input\":\"password\""));
    assert!(contents.contains("\"hash\":\"5f4dcc3b5aa765d61d8327deb882cf99\""));
    fs::remove_file(output_path).unwrap();
}
