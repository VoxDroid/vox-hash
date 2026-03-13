use crate::config::RuntimeConfig;
use crate::domain::candidate_generation::get_charset;
use crate::domain::decryption::BruteForceProvider;
use crate::domain::hashing::{Algorithm, validate_hash};
use crate::domain::matching::{
    CommonPatternsProvider, MatchingOrchestrator, RainbowTableProvider,
    WordlistProvider, MatchProvider,
};
use crate::errors::Result;
use crate::infra::file_io::{read_lines, write_to_file};
use indicatif::{ProgressBar, ProgressStyle};
use rayon::ThreadPoolBuilder;
use rayon::prelude::*;
use serde_json::json;
use std::sync::Arc;

#[allow(clippy::too_many_arguments)]
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
        pb.set_style(
            ProgressStyle::default_bar()
                .template(
                    "{spinner:.green} [{elapsed_precise}] [{bar:40.cyan/blue}] {pos}/{len} ({eta})",
                )
                .unwrap(),
        );
        Some(pb)
    } else {
        None
    };

    let mut orchestrator = MatchingOrchestrator::new();
    if let Some(table_path) = rainbow_table {
        orchestrator.add_provider(Box::new(RainbowTableProvider::new(&table_path)?));
    }
    if common_patterns {
        orchestrator.add_provider(Box::new(CommonPatternsProvider));
    }
    if let Some(wordlist_path) = wordlist {
        orchestrator.add_provider(Box::new(WordlistProvider::new(&wordlist_path, conc)?));
    }

    let orchestrator = Arc::new(orchestrator);
    let charset = get_charset(&config.charset_type, &config.custom_charset);
    let pool = ThreadPoolBuilder::new()
        .num_threads(conc as usize)
        .build()
        .expect("Failed to build thread pool");

    let results: Vec<(String, String)> = pool.install(|| {
        hashes
            .par_chunks(batch_size as usize)
            .flat_map(|batch| {
                batch
                    .par_iter()
                    .filter_map(|hash| {
                        if !validate_hash(hash, algo, auto) {
                            return None;
                        }
                        let hash = hash.trim().to_lowercase();

                        let mut result = orchestrator
                            .find_match(&hash, algo)
                            .ok()
                            .flatten()
                            .map(|(r, _)| r);

                        if result.is_none() {
                            let pb_bf = ProgressBar::hidden();
                            let bf_provider = BruteForceProvider {
                                charset: charset.clone(),
                                min_len,
                                max_len,
                                conc,
                                prefix: prefix.clone(),
                                suffix: suffix.clone(),
                                pattern: pattern.clone(),
                                pb: pb_bf,
                            };
                            result = bf_provider.find_match(&hash, algo).ok().flatten();
                        }

                        if let Some(ref p) = pb_main {
                            p.inc(1);
                        }

                        Some((
                            hash.clone(),
                            result.unwrap_or_else(|| "No match found".to_string()),
                        ))
                    })
                    .collect::<Vec<_>>()
            })
            .collect()
    });

    let output_str = if use_json {
        json!(
            results
                .iter()
                .map(|(hash, result)| json!({ "hash": hash, "result": result }))
                .collect::<Vec<_>>()
        )
        .to_string()
    } else {
        results
            .iter()
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
