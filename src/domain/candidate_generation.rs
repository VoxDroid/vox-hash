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

    let mut expanded_charset = String::new();
    let chars: Vec<char> = caps[1].chars().collect();
    let mut i = 0;
    while i < chars.len() {
        if i + 2 < chars.len() && chars[i + 1] == '-' {
            let start = chars[i] as u8;
            let end = chars[i + 2] as u8;
            if start <= end {
                for c in start..=end {
                    expanded_charset.push(c as char);
                }
                i += 3;
                continue;
            }
        }
        expanded_charset.push(chars[i]);
        i += 1;
    }

    let len: u32 = caps[2]
        .parse()
        .map_err(|_| crate::errors::AppError::Config("Invalid length in pattern".to_string()))?;

    if len > 32 {
        return Err(crate::errors::AppError::Config(
            "Pattern length too large (max 32)".to_string(),
        ));
    }

    Ok((expanded_charset, len))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_get_charset() {
        assert_eq!(get_charset(&CharsetType::Digits, &None), "0123456789");
        assert_eq!(
            get_charset(&CharsetType::Custom, &Some("abc".to_string())),
            "abc"
        );
    }

    #[test]
    fn test_parse_pattern_simple() {
        let (charset, len) = parse_pattern("[abc]{3}").unwrap();
        assert_eq!(charset, "abc");
        assert_eq!(len, 3);
    }

    #[test]
    fn test_parse_pattern_expanded() {
        let (charset, len) = parse_pattern("[a-c1-3]{5}").unwrap();
        assert_eq!(charset, "abc123");
        assert_eq!(len, 5);
    }

    #[test]
    fn test_parse_pattern_invalid() {
        assert!(parse_pattern("abc{3}").is_err());
        assert!(parse_pattern("[abc]3").is_err());
    }
}
