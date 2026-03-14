use crate::config::RuntimeConfig;
use crate::domain::candidate_generation::get_charset;
use crate::domain::decryption::BruteForceProvider;
use crate::domain::hashing::{Algorithm, validate_hash};
use crate::domain::matching::{
    CommonPatternsProvider, MatchProvider, MatchingOrchestrator, RainbowTableProvider,
    WordlistProvider,
};
use crate::errors::Result;
use crate::infra::file_io::{read_lines, write_to_file};
use indicatif::{ProgressBar, ProgressStyle};
use rayon::prelude::*;
use serde_json;
use std::sync::Arc;

use crate::domain::models::BulkDecryptionResult;

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
) -> Result<String> {
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
        orchestrator.add_provider(Box::new(RainbowTableProvider::new(&table_path, algo)?));
    }
    if common_patterns {
        orchestrator.add_provider(Box::new(CommonPatternsProvider));
    }
    if let Some(wordlist_path) = wordlist {
        orchestrator.add_provider(Box::new(WordlistProvider::new(&wordlist_path, conc)?));
    }

    let orchestrator = Arc::new(orchestrator);
    let charset = get_charset(&config.charset_type, &config.custom_charset);
    let pool = crate::infra::concurrency::build_pool(conc);

    let results: Vec<BulkDecryptionResult> = pool.install(|| {
        hashes
            .par_chunks(batch_size as usize)
            .flat_map(|batch| {
                batch
                    .par_iter()
                    .filter_map(|hash| {
                        if crate::infra::shutdown::is_shutdown() {
                            return None;
                        }
                        if !validate_hash(hash, algo, auto) {
                            return None;
                        }
                        let actual_algo = if auto {
                            Algorithm::detect_from_hash(hash).unwrap()
                        } else {
                            algo
                        };
                        let hash = hash.trim().to_lowercase();

                        let mut result = orchestrator
                            .find_match(&hash, actual_algo)
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
                            result = bf_provider.find_match(&hash, actual_algo).ok().flatten();
                        }

                        if let Some(ref p) = pb_main {
                            p.inc(1);
                        }

                        Some(BulkDecryptionResult {
                            hash: hash.clone(),
                            result: result.unwrap_or_else(|| "No match found".to_string()),
                        })
                    })
                    .collect::<Vec<_>>()
            })
            .collect()
    });

    let output_str = if use_json {
        serde_json::to_string(&results)?
    } else {
        results
            .iter()
            .filter(|r| !only_success || r.result != "No match found")
            .map(|r| r.result.as_str())
            .collect::<Vec<_>>()
            .join("\n")
    };

    if let Some(path) = output {
        write_to_file(path, &output_str)?;
    }

    if let Some(p) = pb_main {
        p.finish_with_message("Processing complete");
    }

    Ok(output_str)
}
