mod cli;
mod domain;
mod infra;
mod app;
mod errors;
mod config;

use clap::Parser;
use crate::cli::args::{Cli, Commands};
use crate::config::RuntimeConfig;
use crate::app::enc_use_case::{execute_enc, execute_bulk_enc};
use crate::app::dec_use_case::execute_dec;
use crate::app::bulk_dec_use_case::execute_bulk_dec;
use crate::app::utils_use_case::{execute_benchmark, execute_generate_table};
use crate::domain::candidate_generation::get_charset;
use chrono::Local;

fn main() {
    let cli = Cli::parse();
    let config = RuntimeConfig {
        verbose: !cli.noverbose,
        max_len: cli.max_len,
        charset_type: cli.charset_type,
        custom_charset: cli.charset.clone(),
    };

    if config.verbose {
        println!("Starting vox-hash at {}", Local::now());
    }

    match cli.command {
        Commands::Enc { algo, str, output, json } => {
            match execute_enc(algo, &str, output.as_deref(), json) {
                Ok(hash) => {
                    if config.verbose {
                        println!("Hash: {}", hash);
                    } else if output.is_none() {
                        println!("{}", hash);
                    }
                }
                Err(e) => eprintln!("Error: {}", e),
            }
        }
        Commands::BulkEnc { algo, input, output, json } => {
            if let Err(e) = execute_bulk_enc(algo, &input, output.as_deref(), json) {
                eprintln!("Error: {}", e);
            }
        }
        Commands::Dec { key, auto, algo, conc, wordlist, prefix, suffix, min_len, length, common_patterns, pattern, rainbow_table, output, json } => {
            let (eff_min, eff_max) = match length {
                Some(l) => (l, l),
                None => (min_len, config.max_len),
            };
            
            match execute_dec(key, auto, algo, conc, wordlist, prefix, suffix, eff_min, eff_max, common_patterns, pattern, rainbow_table, &config) {
                Ok(Some(res)) => {
                    if config.verbose {
                        println!("Match found: {}", res);
                    } else if output.is_none() {
                        if json {
                            println!("{}", serde_json::json!({"result": res}));
                        } else {
                            println!("{}", res);
                        }
                    }
                }
                Ok(None) => println!("No match found within constraints"),
                Err(e) => eprintln!("Error: {}", e),
            }
        }
        Commands::BulkDec { input, auto, algo, conc, wordlist, prefix, suffix, min_len, length, common_patterns, pattern, rainbow_table, output, json, batch_size, only_success } => {
            let (eff_min, eff_max) = match length {
                Some(l) => (l, l),
                None => (min_len, config.max_len),
            };
            
            if let Err(e) = execute_bulk_dec(
                &input, auto, algo, conc, wordlist, prefix, suffix, eff_min, eff_max, common_patterns, pattern, rainbow_table, output, json, batch_size, only_success, &config
            ) {
                eprintln!("Error: {}", e);
            }
        }
        Commands::GenerateTable { output, min_len, max_len, algo } => {
            let charset = get_charset(&config.charset_type, &config.custom_charset);
            if let Err(e) = execute_generate_table(algo, &charset, min_len, max_len, &output, config.verbose) {
                eprintln!("Error: {}", e);
            }
        }
        Commands::Benchmark { algo, iterations } => {
            let hps = execute_benchmark(algo, iterations);
            println!("{:.2} hashes/second", hps);
        }
    }
}