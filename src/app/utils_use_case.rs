use crate::domain::hashing::{Algorithm, hash_string};
use crate::errors::Result;
use indicatif::{ProgressBar, ProgressStyle};
use serde_json::json;
use std::fs::OpenOptions;
use std::time::Instant;

pub fn execute_benchmark(algo: Algorithm, iterations: u32) -> f64 {
    let start = Instant::now();
    for _ in 0..iterations {
        hash_string("test", algo);
    }
    let elapsed = start.elapsed().as_secs_f64();
    (iterations as f64) / elapsed
}

pub fn execute_generate_table(
    algo: Algorithm,
    charset: &str,
    min_len: u32,
    max_len: u32,
    output: &str,
    verbose: bool,
) -> Result<()> {
    let charset_chars: Vec<char> = charset.chars().collect();
    let mut table = serde_json::Map::new();

    let pb = if verbose {
        let mut total: u64 = 0;
        for len in min_len..=max_len {
            total += (charset_chars.len() as u64).pow(len);
        }
        let pb = ProgressBar::new(total);
        pb.set_style(
            ProgressStyle::default_bar()
                .template(
                    "{spinner:.green} [{elapsed_precise}] [{bar:40.cyan/blue}] {pos}/{len} ({eta})",
                )
                .unwrap(),
        );
        Some(pb)
    } else {
        None
    };

    for len in min_len..=max_len {
        let total_for_len = (charset_chars.len() as u64).pow(len);
        for n in 0..total_for_len {
            let mut idx = vec![0; len as usize];
            let mut temp_n = n;
            let c_len = charset_chars.len() as u64;
            for i in (0..len as usize).rev() {
                idx[i] = (temp_n % c_len) as usize;
                temp_n /= c_len;
            }
            let candidate: String = idx.iter().map(|&i| charset_chars[i]).collect();
            let hash = hash_string(&candidate, algo);
            table.insert(hash, json!(candidate));
            if let Some(ref p) = pb {
                p.inc(1);
            }
        }
    }

    let file = OpenOptions::new()
        .write(true)
        .create(true)
        .truncate(true)
        .open(output)?;
    serde_json::to_writer(file, &table)?;

    if let Some(p) = pb {
        p.finish_with_message("Table generated");
    }

    Ok(())
}
