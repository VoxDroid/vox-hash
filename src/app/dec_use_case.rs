use crate::config::RuntimeConfig;
use crate::domain::candidate_generation::get_charset;
use crate::domain::decryption::BruteForceProvider;
use crate::domain::hashing::{Algorithm, validate_hash};
use crate::domain::matching::{
    CommonPatternsProvider, MatchingOrchestrator, RainbowTableProvider, WordlistProvider,
};
use crate::domain::models::DecryptionResult;
use crate::errors::{AppError, Result};
use indicatif::ProgressBar;

#[allow(clippy::too_many_arguments)]
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
) -> Result<Option<DecryptionResult>> {
    if !validate_hash(&key, algo, auto) {
        return Err(AppError::InvalidHash(key));
    }
    let algo = if auto {
        Algorithm::detect_from_hash(&key).unwrap() // Validated above
    } else {
        algo
    };
    let key = key.trim().to_lowercase();

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

    let charset = get_charset(&config.charset_type, &config.custom_charset);
    let pb = if config.verbose {
        let pb = ProgressBar::new_spinner();
        pb.set_style(
            indicatif::ProgressStyle::default_spinner()
                .template("{spinner:.green} [{elapsed_precise}] {msg} {pos} candidates...")
                .unwrap(),
        );
        pb.set_message("Brute-forcing:");
        pb
    } else {
        ProgressBar::hidden()
    };

    orchestrator.add_provider(Box::new(BruteForceProvider {
        charset,
        min_len,
        max_len,
        conc,
        prefix,
        suffix,
        pattern,
        pb: pb.clone(),
    }));

    let (result, stats) = orchestrator.find_match_with_stats(&key, algo)?;
    pb.finish_and_clear();

    if config.verbose {
        println!("Matching stats:");
        for (name, time) in stats.provider_times {
            println!("  - {}: {:.2?}", name, time);
        }
        println!("  Total time: {:.2?}", stats.total_time);
    }

    Ok(result.map(|(res, _)| DecryptionResult {
        hash: key,
        result: res,
    }))
}
