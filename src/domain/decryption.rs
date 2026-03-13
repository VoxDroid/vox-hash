use crate::domain::hashing::{hash_string, Algorithm};
use crate::domain::candidate_generation::parse_pattern;
use indicatif::ProgressBar;
use rayon::prelude::*;
use rayon::ThreadPoolBuilder;
use std::sync::atomic::{AtomicBool, Ordering};
use std::time::Instant;

pub fn brute_force_hash(
    target: &str,
    algo: Algorithm,
    charset: &str,
    min_len: u32,
    max_len: u32,
    conc: u32,
    prefix: &str,
    suffix: &str,
    pattern: Option<&str>,
    verbose: bool,
    pb: &ProgressBar,
) -> Option<String> {
    let start_time = Instant::now();
    let charset_chars: Vec<char> = charset.chars().collect();
    let _charset_len = charset_chars.len() as u64;
    let fixed_len = prefix.len() as u32 + suffix.len() as u32;

    if min_len < fixed_len {
        return None;
    }
    if min_len > max_len {
        return None;
    }

    let (effective_charset, effective_min_len, effective_max_len) = if let Some(pattern) = pattern {
        let (pat_charset, len) = parse_pattern(pattern)?;
        (pat_charset.chars().collect::<Vec<char>>(), len, len)
    } else {
        (charset_chars, min_len, max_len)
    };

    let found_flag = AtomicBool::new(false);
    let pool = ThreadPoolBuilder::new().num_threads(conc as usize).build().expect("Failed to build thread pool");
    
    let result = pool.install(|| {
        for len in effective_min_len..=effective_max_len {
            if found_flag.load(Ordering::Relaxed) {
                break;
            }
            let var_len = len as u64 - fixed_len as u64;
            if var_len <= 0 {
                if prefix.len() + suffix.len() == len as usize {
                   let candidate = format!("{}{}", prefix, suffix);
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
                    let candidate = format!("{}{}{}", prefix, middle, suffix);
                    
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

    if verbose && result.is_some() {
        println!("Time taken: {:?}", start_time.elapsed());
    }
    result
}
