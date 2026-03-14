use crate::domain::hashing::{Algorithm, hash_string};
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
        json!({ "result": result }).to_string()
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
    let results: Vec<(String, String)> = lines
        .into_iter()
        .map(|line| {
            let hash = hash_string(&line, algo);
            (line, hash)
        })
        .collect();

    let output_str = if use_json {
        json!(
            results
                .iter()
                .map(|(input, hash)| json!({ "input": input, "hash": hash }))
                .collect::<Vec<_>>()
        )
        .to_string()
    } else {
        results
            .iter()
            .map(|(_, hash)| hash.as_str())
            .collect::<Vec<_>>()
            .join("\n")
    };

    if let Some(path) = output {
        write_to_file(path, &output_str)?;
    }

    Ok(output_str)
}
