use assert_cmd::Command;
use std::fs;

#[test]
fn test_generate_table_basic() {
    let output_path = "tests/data/test_table.json";
    let mut cmd = Command::cargo_bin("vox-hash").unwrap();
    cmd.arg("generate-table")
        .arg("--output")
        .arg(output_path)
        .arg("--min-len")
        .arg("1")
        .arg("--max-len")
        .arg("2")
        .arg("--algo")
        .arg("md5")
        .arg("--noverbose");
    cmd.assert().success();

    let contents = fs::read_to_string(output_path).unwrap();
    assert!(contents.contains("\"version\":\"1.0\""));
    assert!(contents.contains("\"algo\":\"md5\""));
    assert!(contents.contains("0cc175b9c0f1b6a831c399e269772661")); // "a"
    fs::remove_file(output_path).unwrap();
}
