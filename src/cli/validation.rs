use crate::cli::args::Commands;
use crate::config::RuntimeConfig;
use crate::errors::{AppError, Result};

pub fn validate_cli_args(command: &Commands, config: &RuntimeConfig) -> Result<()> {
    match command {
        Commands::Dec {
            min_len,
            prefix,
            suffix,
            pattern,
            ..
        }
        | Commands::BulkDec {
            min_len,
            prefix,
            suffix,
            pattern,
            ..
        } => {
            let fixed_len = (prefix.len() + suffix.len()) as u32;
            if *min_len < fixed_len {
                return Err(AppError::Config(format!(
                    "--min-len ({}) must be >= length of prefix ({}) + suffix ({})",
                    min_len,
                    prefix.len(),
                    suffix.len()
                )));
            }
            if *min_len > config.max_len {
                return Err(AppError::Config(format!(
                    "--min-len ({}) must be <= --max-len ({})",
                    min_len, config.max_len
                )));
            }
            if let Some(p) = pattern {
                crate::domain::candidate_generation::parse_pattern(p)?;
            }
        }
        Commands::GenerateTable {
            min_len, max_len, ..
        } => {
            if *min_len > *max_len {
                return Err(AppError::Config(format!(
                    "--min-len ({}) must be <= --max-len ({})",
                    min_len, max_len
                )));
            }
        }
        _ => {}
    }
    Ok(())
}
