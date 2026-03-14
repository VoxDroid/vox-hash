use crate::domain::hashing::{Algorithm, hash_string};
use crate::domain::models::HashResult;
use crate::errors::Result;
use crate::infra::file_io::{read_lines, write_to_file};
use serde_json::json;

pub fn execute_enc(
    algo: Algorithm,
    input: &str,
    output: Option<&str>,
    use_json: bool,
) -> Result<String> {
    let result = hash_string(input, algo);
    let output_str = if use_json {
        json!({ "hash": result }).to_string()
    } else {
        result.clone()
    };

    if let Some(path) = output {
        write_to_file(path, &output_str)?;
    }

    Ok(output_str)
}

pub fn execute_bulk_enc(
    algo: Algorithm,
    input_path: &str,
    output: Option<&str>,
    use_json: bool,
) -> Result<String> {
    let lines = read_lines(input_path)?;
    let results: Vec<HashResult> = lines
        .into_iter()
        .map(|line| {
            let hash = hash_string(&line, algo);
            HashResult { input: line, hash }
        })
        .collect();

    let output_str = if use_json {
        serde_json::to_string(&results)?
    } else {
        results
            .iter()
            .map(|r| r.hash.as_str())
            .collect::<Vec<_>>()
            .join("\n")
    };

    if let Some(path) = output {
        write_to_file(path, &output_str)?;
    }

    Ok(output_str)
}
