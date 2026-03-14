use crate::domain::hashing::{Algorithm, hash_string};
use crate::errors::{AppError, Result};
use crate::infra::file_io::read_lines;
use rayon::ThreadPoolBuilder;
use rayon::prelude::*;
use serde_json::Value;
use std::fs::File;
use std::path::Path;

pub trait MatchProvider: Send + Sync {
    fn name(&self) -> &str;
    fn find_match(&self, target: &str, algo: Algorithm) -> Result<Option<String>>;
}

pub struct CommonPatternsProvider;
impl MatchProvider for CommonPatternsProvider {
    fn name(&self) -> &str {
        "common_patterns"
    }
    fn find_match(&self, target: &str, algo: Algorithm) -> Result<Option<String>> {
        let common = vec![
            "password",
            "admin",
            "123456",
            "qwerty",
            "letmein",
            "welcome",
            "abc123",
            "password1",
            "test123",
            "admin123",
            "user",
            "guest",
            "passkord",
        ];
        for word in common {
            if hash_string(word, algo) == target {
                return Ok(Some(word.to_string()));
            }
        }
        Ok(None)
    }
}

pub struct WordlistProvider {
    pub words: Vec<String>,
    pub conc: u32,
}
impl WordlistProvider {
    pub fn new(path: &str, conc: u32) -> Result<Self> {
        let words = read_lines(path)?;
        Ok(Self { words, conc })
    }
}
impl MatchProvider for WordlistProvider {
    fn name(&self) -> &str {
        "wordlist"
    }
    fn find_match(&self, target: &str, algo: Algorithm) -> Result<Option<String>> {
        let pool = ThreadPoolBuilder::new()
            .num_threads(self.conc as usize)
            .build()
            .map_err(|e| AppError::Config(format!("Failed to build thread pool: {}", e)))?;

        let result = pool.install(|| {
            self.words.par_iter().find_map_any(|word| {
                if hash_string(word, algo) == target {
                    Some(word.clone())
                } else {
                    None
                }
            })
        });
        Ok(result)
    }
}

pub struct RainbowTableProvider {
    pub table: serde_json::Map<String, Value>,
}
impl RainbowTableProvider {
    pub fn new(path: &str, algo: Algorithm) -> Result<Self> {
        if !Path::new(path).exists() {
            return Err(AppError::NotFound(format!(
                "Rainbow table file '{}' does not exist",
                path
            )));
        }
        let file = File::open(path)?;
        let value: Value = serde_json::from_reader(file)?;
        let obj = value
            .as_object()
            .ok_or_else(|| AppError::Config("Rainbow table is not a JSON object".to_string()))?;

        let version = obj
            .get("version")
            .and_then(|v| v.as_str())
            .ok_or_else(|| AppError::Config("Missing version in rainbow table".to_string()))?;

        if version != "1.0" {
            return Err(AppError::Config(format!(
                "Unsupported rainbow table version: {}",
                version
            )));
        }

        let table_algo = obj
            .get("algo")
            .and_then(|v| v.as_str())
            .ok_or_else(|| AppError::Config("Missing algorithm in rainbow table".to_string()))?;

        let expected_algo_str = format!("{:?}", algo).to_lowercase();
        if table_algo != expected_algo_str {
            return Err(AppError::Config(format!(
                "Rainbow table algorithm mismatch. Expected {}, found {}",
                expected_algo_str, table_algo
            )));
        }

        let table = obj
            .get("table")
            .and_then(|v| v.as_object())
            .ok_or_else(|| AppError::Config("Missing or invalid table object".to_string()))?
            .clone();

        Ok(Self { table })
    }
}
impl MatchProvider for RainbowTableProvider {
    fn name(&self) -> &str {
        "rainbow_table"
    }
    fn find_match(&self, target: &str, _algo: Algorithm) -> Result<Option<String>> {
        Ok(self
            .table
            .get(target)
            .and_then(|v| v.as_str().map(String::from)))
    }
}
use std::time::{Duration, Instant};

pub struct MatchingStats {
    pub provider_times: Vec<(String, Duration)>,
    pub total_time: Duration,
}

pub struct MatchingOrchestrator {
    pub providers: Vec<Box<dyn MatchProvider>>,
}

impl MatchingOrchestrator {
    pub fn new() -> Self {
        Self {
            providers: Vec::new(),
        }
    }

    pub fn add_provider(&mut self, provider: Box<dyn MatchProvider>) {
        self.providers.push(provider);
    }

    pub fn find_match_with_stats(
        &self,
        target: &str,
        algo: Algorithm,
    ) -> Result<(Option<(String, &str)>, MatchingStats)> {
        let mut provider_times = Vec::new();
        let total_start = Instant::now();

        for provider in &self.providers {
            let start = Instant::now();
            let res = provider.find_match(target, algo)?;
            let elapsed = start.elapsed();
            provider_times.push((provider.name().to_string(), elapsed));

            if let Some(res_str) = res {
                return Ok((
                    Some((res_str, provider.name())),
                    MatchingStats {
                        provider_times,
                        total_time: total_start.elapsed(),
                    },
                ));
            }
        }

        Ok((
            None,
            MatchingStats {
                provider_times,
                total_time: total_start.elapsed(),
            },
        ))
    }

    pub fn find_match(&self, target: &str, algo: Algorithm) -> Result<Option<(String, &str)>> {
        let (res, _) = self.find_match_with_stats(target, algo)?;
        Ok(res)
    }
}
