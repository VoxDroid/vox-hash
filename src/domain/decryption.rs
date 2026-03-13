use crate::domain::candidate_generation::parse_pattern;
use crate::domain::hashing::{Algorithm, hash_string};
use crate::domain::matching::MatchProvider;
use crate::errors::Result;
use indicatif::ProgressBar;
use rayon::ThreadPoolBuilder;
use rayon::prelude::*;
use std::sync::atomic::{AtomicBool, Ordering};
use std::time::Instant;

pub struct BruteForceProvider {
    pub charset: String,
    pub min_len: u32,
    pub max_len: u32,
    pub conc: u32,
    pub prefix: String,
    pub suffix: String,
    pub pattern: Option<String>,
    pub pb: ProgressBar,
}

pub struct BruteForceOptions<'a> {
    pub charset: &'a str,
    pub min_len: u32,
    pub max_len: u32,
    pub conc: u32,
    pub prefix: &'a str,
    pub suffix: &'a str,
    pub pattern: Option<&'a str>,
    pub verbose: bool,
}

impl MatchProvider for BruteForceProvider {
    fn name(&self) -> &str { "brute_force" }
    fn find_match(&self, target: &str, algo: Algorithm) -> Result<Option<String>> {
        let opts = BruteForceOptions {
            charset: &self.charset,
            min_len: self.min_len,
            max_len: self.max_len,
            conc: self.conc,
            prefix: &self.prefix,
            suffix: &self.suffix,
            pattern: self.pattern.as_deref(),
            verbose: false,
        };
        brute_force_hash(target, algo, opts, &self.pb)
    }
}

pub fn brute_force_hash(
    target: &str,
    algo: Algorithm,
    opts: BruteForceOptions,
    pb: &ProgressBar,
) -> Result<Option<String>> {
    let start_time = Instant::now();
    let charset_chars: Vec<char> = opts.charset.chars().collect();
    let fixed_len = opts.prefix.len() as u32 + opts.suffix.len() as u32;

    if opts.min_len < fixed_len {
        return Ok(None);
    }
    if opts.min_len > opts.max_len {
        return Ok(None);
    }

    let (effective_charset, effective_min_len, effective_max_len) = if let Some(pattern) = opts.pattern {
        let (pat_charset, len) = parse_pattern(pattern)?;
        (pat_charset.chars().collect::<Vec<char>>(), len, len)
    } else {
        (charset_chars, opts.min_len, opts.max_len)
    };

    let found_flag = AtomicBool::new(false);
    let pool = ThreadPoolBuilder::new().num_threads(opts.conc as usize).build().expect("Failed to build thread pool");
    
    let result = pool.install(|| {
        for len in effective_min_len..=effective_max_len {
            if found_flag.load(Ordering::Relaxed) {
                break;
            }
            let var_len = len as u64 - fixed_len as u64;
            if var_len == 0 {
                if opts.prefix.len() + opts.suffix.len() == len as usize {
                   let candidate = format!("{}{}", opts.prefix, opts.suffix);
                   if hash_string(&candidate, algo) == target {
                       found_flag.store(true, Ordering::Relaxed);
                       return Some(candidate);
                   }
                }
                continue;
            }
            let total_for_len = (effective_charset.len() as u64).pow(var_len as u32);
            let batch_size = 1_000_000;
            let mut start: u64 = 0;
            while start < total_for_len {
                if found_flag.load(Ordering::Relaxed) {
                    break;
                }
                let end = (start + batch_size).min(total_for_len);
                let found = (start..end).into_par_iter().find_map_any(|n| {
                    if found_flag.load(Ordering::Relaxed) {
                        return None;
                    }
                    let mut idx = vec![0; var_len as usize];
                    let mut temp_n = n;
                    let c_len = effective_charset.len() as u64;
                    for i in (0..var_len as usize).rev() {
                        idx[i] = (temp_n % c_len) as usize;
                        temp_n /= c_len;
                    }
                    let middle: String = idx.iter().map(|&i| effective_charset[i]).collect();
                    let candidate = format!("{}{}{}", opts.prefix, middle, opts.suffix);
                    
                    pb.inc(1);
                    if hash_string(&candidate, algo) == target {
                        found_flag.store(true, Ordering::Relaxed);
                        Some(candidate)
                    } else {
                        None
                    }
                });
                if let Some(found) = found {
                    return Some(found);
                }
                start += batch_size;
            }
        }
        None
    });

    if opts.verbose && result.is_some() {
        println!("Time taken: {:?}", start_time.elapsed());
    }
    Ok(result)
}
