use crate::domain::hashing::{Algorithm, validate_hash, hash_string};
use crate::app::dec_use_case::{try_rainbow_table, try_common_patterns};
use crate::domain::decryption::brute_force_hash;
use crate::domain::candidate_generation::get_charset;
use crate::infra::file_io::{read_lines, write_to_file};
use crate::errors::Result;
use crate::config::RuntimeConfig;
use indicatif::{ProgressBar, ProgressStyle};
use rayon::prelude::*;
use rayon::ThreadPoolBuilder;
use serde_json::json;

pub fn execute_bulk_dec(
    input_path: &str,
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
    output: Option<String>,
    use_json: bool,
    batch_size: u32,
    only_success: bool,
    config: &RuntimeConfig,
) -> Result<Vec<(String, String)>> {
    let hashes = read_lines(input_path)?;
    let total = hashes.len() as u64;
    
    let pb_main = if config.verbose {
        let pb = ProgressBar::new(total);
        pb.set_style(ProgressStyle::default_bar().template("{spinner:.green} [{elapsed_precise}] [{bar:40.cyan/blue}] {pos}/{len} ({eta})").unwrap());
        Some(pb)
    } else {
        None
    };

    let charset = get_charset(&config.charset_type, &config.custom_charset);
    let pool = ThreadPoolBuilder::new().num_threads(conc as usize).build().expect("Failed to build thread pool");

    let results: Vec<(String, String)> = pool.install(|| {
        hashes.par_chunks(batch_size as usize).flat_map(|batch| {
            batch.par_iter().filter_map(|hash| {
                if !validate_hash(hash, algo, auto) {
                    return None;
                }
                
                let mut result = None;
                
                if let Some(table_path) = &rainbow_table {
                    result = try_rainbow_table(hash, table_path).ok().flatten();
                }
                
                if result.is_none() && common_patterns {
                    result = try_common_patterns(hash, algo);
                }
                
                if result.is_none() {
                    if let Some(wordlist_path) = &wordlist {
                        // This is inefficient to read wordlist every time, but keeping it simple for now
                        // In a real refactor, we'd load it once. 
                        // Actually, let's just assume we'd refactor wordlist loading later.
                        if let Ok(words) = read_lines(wordlist_path) {
                             result = words.iter().find(|&w| hash_string(w, algo) == *hash).cloned();
                        }
                    }
                }
                
                if result.is_none() {
                    let pb_bf = ProgressBar::hidden();
                    result = brute_force_hash(
                        hash,
                        algo,
                        &charset,
                        min_len,
                        max_len,
                        conc,
                        &prefix,
                        &suffix,
                        pattern.as_deref(),
                        false,
                        &pb_bf,
                    );
                }
                
                if let Some(ref p) = pb_main {
                    p.inc(1);
                }
                
                Some((hash.clone(), result.unwrap_or_else(|| "No match found".to_string())))
            }).collect::<Vec<_>>()
        }).collect()
    });

    let output_str = if use_json {
        json!(results.iter().map(|(hash, result)| json!({ "hash": hash, "result": result })).collect::<Vec<_>>()).to_string()
    } else {
        results.iter()
            .filter(|(_, result)| !only_success || result != "No match found")
            .map(|(_, result)| result.as_str())
            .collect::<Vec<_>>()
            .join("\n")
    };

    if let Some(path) = output {
        write_to_file(path, &output_str)?;
    }

    if let Some(p) = pb_main {
        p.finish_with_message("Processing complete");
    }

    Ok(results)
}
