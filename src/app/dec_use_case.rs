use crate::domain::hashing::{hash_string, Algorithm, validate_hash};
use crate::domain::decryption::brute_force_hash;
use crate::domain::candidate_generation::get_charset;
use crate::infra::file_io::{read_lines};
use crate::errors::{AppError, Result};
use crate::config::RuntimeConfig;
use indicatif::ProgressBar;
use serde_json::Value;
use std::fs::File;
use std::path::Path;
use rayon::prelude::*;
use rayon::ThreadPoolBuilder;

pub fn try_common_patterns(target: &str, algo: Algorithm) -> Option<String> {
    let common = vec![
        "password", "admin", "123456", "qwerty", "letmein", "welcome",
        "abc123", "password1", "test123", "admin123", "user", "guest",
        "passkord",
    ];
    for word in common {
        if hash_string(word, algo) == target {
            return Some(word.to_string());
        }
    }
    None
}

pub fn try_rainbow_table(target: &str, table_path: &str) -> Result<Option<String>> {
    if !Path::new(table_path).exists() {
        return Err(AppError::NotFound(format!("Rainbow table file '{}' does not exist", table_path)));
    }
    let file = File::open(table_path)?;
    let table: Value = serde_json::from_reader(file)?;
    let obj = table.as_object().ok_or_else(|| AppError::Config("Rainbow table is not a JSON object".to_string()))?;
    Ok(obj.get(target).and_then(|v| v.as_str().map(String::from)))
}

pub fn execute_dec(
    key: String,
    auto: bool,
    algo: Algorithm,
    conc: u32,
    wordlist: Option<String>,
    prefix: String,
    suffix: String,
    min_len: u32,
    max_len: u32,
    common_patterns: bool,
    pattern: Option<String>,
    rainbow_table: Option<String>,
    config: &RuntimeConfig,
) -> Result<Option<String>> {
    if !validate_hash(&key, algo, auto) {
        return Err(AppError::InvalidHash(key));
    }

    let mut result = None;

    if let Some(table_path) = &rainbow_table {
        result = try_rainbow_table(&key, table_path)?;
    }

    if result.is_none() && common_patterns {
        result = try_common_patterns(&key, algo);
    }

    if result.is_none() {
        if let Some(wordlist_path) = &wordlist {
            let words = read_lines(wordlist_path)?;
            let pool = ThreadPoolBuilder::new().num_threads(conc as usize).build().expect("Failed to build thread pool");
            result = pool.install(|| {
                words.par_iter().find_map_any(|word| {
                    if hash_string(word, algo) == key {
                        Some(word.clone())
                    } else {
                        None
                    }
                })
            });
        }
    }

    if result.is_none() {
        let charset = get_charset(&config.charset_type, &config.custom_charset);
        let pb = if config.verbose {
            ProgressBar::new_spinner()
        } else {
            ProgressBar::hidden()
        };
        result = brute_force_hash(
            &key,
            algo,
            &charset,
            min_len,
            max_len,
            conc,
            &prefix,
            &suffix,
            pattern.as_deref(),
            config.verbose,
            &pb,
        );
        pb.finish_and_clear();
    }

    Ok(result)
}
