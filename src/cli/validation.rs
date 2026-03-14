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
                let (pat_charset, pat_len) = crate::domain::candidate_generation::parse_pattern(p)?;
                let workload = (pat_charset.chars().count() as u128).pow(pat_len);
                if workload > 1_000_000_000_000 {
                    return Err(AppError::Config(format!(
                        "Pattern workload too large: {} candidates. Max allowed is 1,000,000,000,000.",
                        workload
                    )));
                }
            } else {
                let charset = crate::domain::candidate_generation::get_charset(
                    &config.charset_type,
                    &config.custom_charset,
                );
                let c_size = charset.chars().count() as u128;
                let var_len = config.max_len.saturating_sub(fixed_len);
                if var_len > 0 {
                    let workload = c_size.pow(var_len);
                    if workload > 1_000_000_000_000 {
                        return Err(AppError::Config(format!(
                            "Brute force workload too large: {} candidates. Max allowed is 1,000,000,000,000. Try reducing --max-len or using a pattern.",
                            workload
                        )));
                    }
                }
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

#[cfg(test)]
mod tests {
    use super::*;
    use crate::domain::candidate_generation::CharsetType;
    use crate::domain::hashing::Algorithm;

    #[test]
    fn test_validate_cli_args_success() {
        let config = RuntimeConfig {
            verbose: true,
            max_len: 6,
            charset_type: CharsetType::Alphanumeric,
            custom_charset: None,
        };
        let cmd = Commands::Dec {
            key: "5baa61e4c9b93f3f0682250b6cf8331b7ee68fd8".to_string(),
            auto: false,
            algo: Algorithm::Sha1,
            conc: 1,
            wordlist: None,
            prefix: "".to_string(),
            suffix: "".to_string(),
            min_len: 1,
            length: None,
            common_patterns: false,
            pattern: None,
            rainbow_table: None,
            output: None,
            json: false,
        };
        assert!(validate_cli_args(&cmd, &config).is_ok());
    }

    #[test]
    fn test_validate_cli_args_min_len_error() {
        let config = RuntimeConfig {
            verbose: true,
            max_len: 4,
            charset_type: CharsetType::Alphanumeric,
            custom_charset: None,
        };
        let cmd = Commands::Dec {
            key: "key".to_string(),
            auto: false,
            algo: Algorithm::Sha1,
            conc: 1,
            wordlist: None,
            prefix: "".to_string(),
            suffix: "".to_string(),
            min_len: 5,
            length: None,
            common_patterns: false,
            pattern: None,
            rainbow_table: None,
            output: None,
            json: false,
        };
        assert!(validate_cli_args(&cmd, &config).is_err());
    }
}
