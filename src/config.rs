use crate::domain::candidate_generation::CharsetType;

#[derive(Debug, Clone)]
pub struct RuntimeConfig {
    pub verbose: bool,
    pub max_len: u32,
    pub charset_type: CharsetType,
    pub custom_charset: Option<String>,
}

impl Default for RuntimeConfig {
    fn default() -> Self {
        Self {
            verbose: true,
            max_len: 6,
            charset_type: CharsetType::Alphanumeric,
            custom_charset: None,
        }
    }
}
