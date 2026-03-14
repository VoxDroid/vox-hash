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

#[cfg(test)]
mod tests {
    use super::*;
    use crate::domain::candidate_generation::CharsetType;
    use crate::domain::hashing::Algorithm;

    #[test]
    fn test_validate_cli_args_success() {
        let config = RuntimeConfig {
            verbose: true,
            max_len: 10,
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
