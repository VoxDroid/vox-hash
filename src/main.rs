mod app;
mod cli;
mod config;
mod domain;
mod errors;
mod infra;

use crate::app::bulk_dec_use_case::execute_bulk_dec;
use crate::app::dec_use_case::execute_dec;
use crate::app::enc_use_case::{execute_bulk_enc, execute_enc};
use crate::app::utils_use_case::{execute_benchmark, execute_generate_table};
use crate::cli::args::{Cli, Commands};
use crate::cli::validation::validate_cli_args;
use crate::config::RuntimeConfig;
use crate::domain::candidate_generation::get_charset;
use crate::errors::AppError;
use chrono::Local;
use clap::Parser;
use std::process::ExitCode;

fn run() -> Result<ExitCode, AppError> {
    let cli = Cli::parse();
    let config = RuntimeConfig {
        verbose: !cli.noverbose,
        max_len: cli.max_len,
        charset_type: cli.charset_type,
        custom_charset: cli.charset.clone(),
    };

    validate_cli_args(&cli.command, &config)?;

    if config.verbose {
        println!("Starting vox-hash at {}", Local::now());
    }

    match cli.command {
        Commands::Enc {
            algo,
            str,
            output,
            json,
        } => {
            let hash = execute_enc(algo, &str, output.as_deref(), json)?;
            if config.verbose {
                println!("Hash: {}", hash);
            } else if output.is_none() {
                println!("{}", hash);
            }
        }
        Commands::BulkEnc {
            algo,
            input,
            output,
            json,
        } => {
            execute_bulk_enc(algo, &input, output.as_deref(), json)?;
        }
        Commands::Dec {
            key,
            auto,
            algo,
            conc,
            wordlist,
            prefix,
            suffix,
            min_len,
            length,
            common_patterns,
            pattern,
            rainbow_table,
            output,
            json,
        } => {
            let (eff_min, eff_max) = match length {
                Some(l) => (l, l),
                None => (min_len, config.max_len),
            };

            match execute_dec(
                key,
                auto,
                algo,
                conc,
                wordlist,
                prefix,
                suffix,
                eff_min,
                eff_max,
                common_patterns,
                pattern,
                rainbow_table,
                &config,
            )? {
                Some(res) => {
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
                None => {
                    println!("No match found within constraints");
                    return Ok(ExitCode::from(1));
                }
            }
        }
        Commands::BulkDec {
            input,
            auto,
            algo,
            conc,
            wordlist,
            prefix,
            suffix,
            min_len,
            length,
            common_patterns,
            pattern,
            rainbow_table,
            output,
            json,
            batch_size,
            only_success,
        } => {
            let (eff_min, eff_max) = match length {
                Some(l) => (l, l),
                None => (min_len, config.max_len),
            };

            execute_bulk_dec(
                &input,
                auto,
                algo,
                conc,
                wordlist,
                prefix,
                suffix,
                eff_min,
                eff_max,
                common_patterns,
                pattern,
                rainbow_table,
                output,
                json,
                batch_size,
                only_success,
                &config,
            )?;
        }
        Commands::GenerateTable {
            output,
            min_len,
            max_len,
            algo,
        } => {
            let charset = get_charset(&config.charset_type, &config.custom_charset);
            execute_generate_table(algo, &charset, min_len, max_len, &output, config.verbose)?;
        }
        Commands::Benchmark { algo, iterations } => {
            let hps = execute_benchmark(algo, iterations);
            println!("{:.2} hashes/second", hps);
        }
    }

    Ok(ExitCode::SUCCESS)
}

fn main() -> ExitCode {
    match run() {
        Ok(code) => code,
        Err(e) => {
            eprintln!("Error: {}", e);
            match e {
                AppError::InvalidHash(_) => ExitCode::from(2),
                AppError::Config(_) => ExitCode::from(3),
                AppError::NotFound(_) => ExitCode::from(4),
                _ => ExitCode::from(1),
            }
        }
    }
}
