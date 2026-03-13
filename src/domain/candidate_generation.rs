use clap::ValueEnum;
use regex::Regex;

#[derive(Clone, ValueEnum, Debug, Copy, PartialEq, Eq)]
pub enum CharsetType {
    Alphanumeric,
    Lowercase,
    Uppercase,
    Digits,
    Custom,
}

pub fn get_charset(charset_type: &CharsetType, custom_charset: &Option<String>) -> String {
    match charset_type {
        CharsetType::Alphanumeric => "abcdefghijklmnopqrstuvwxyz0123456789".to_string(),
        CharsetType::Lowercase => "abcdefghijklmnopqrstuvwxyz".to_string(),
        CharsetType::Uppercase => "ABCDEFGHIJKLMNOPQRSTUVWXYZ".to_string(),
        CharsetType::Digits => "0123456789".to_string(),
        CharsetType::Custom => custom_charset
            .as_ref()
            .map_or("abcdefghijklmnopqrstuvwxyz0123456789".to_string(), |s| {
                s.clone()
            }),
    }
}

pub fn parse_pattern(pattern: &str) -> crate::errors::Result<(String, u32)> {
    let re =
        Regex::new(r"^\[([a-zA-Z0-9!@#$%^&*()_+\-=\[\]{};':\x22\\|,.<>/?]+)\]\{(\d+)\}$").unwrap();
    let caps = re.captures(pattern).ok_or_else(|| {
        crate::errors::AppError::Config(format!(
            "Invalid pattern format: {}. Expected '[charset]{{len}}'",
            pattern
        ))
    })?;

    let charset = caps[1].to_string();
    let len: u32 = caps[2]
        .parse()
        .map_err(|_| crate::errors::AppError::Config("Invalid length in pattern".to_string()))?;

    if len > 32 {
        return Err(crate::errors::AppError::Config(
            "Pattern length too large (max 32)".to_string(),
        ));
    }

    Ok((charset, len))
}
