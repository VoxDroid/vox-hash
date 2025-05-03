use clap::{Parser, Subcommand};
use sha1::{Digest, Sha1};
use md5::compute as md5_compute;
use indicatif::{ProgressBar, ProgressStyle};
use chrono::Local;
use std::time::Instant;
use rayon::prelude::*;
use rayon::ThreadPoolBuilder;
use std::fs::File;
use std::io::{BufRead, BufReader};

#[derive(Parser)]
#[clap(name = "vox-hash", version = "1.0", about = "A CLI tool for SHA1 and MD5 hashing and brute-force hash matching")]
struct Cli {
    #[clap(subcommand)]
    command: Commands,

    #[clap(long, global = true)]
    noverbose: bool,

    #[clap(long, default_value = "6")]
    max_len: usize,

    #[clap(long, default_value = "abcdefghijklmnopqrstuvwxyz0123456789")]
    charset: String,
}

#[derive(Subcommand)]
enum Commands {
    Enc {
        #[clap(long, value_enum)]
        algo: Algorithm,

        #[clap(long)]
        str: String,
    },
    Dec {
        #[clap(long)]
        key: String,

        #[clap(long)]
        auto: bool,

        #[clap(long, value_enum, default_value = "sha1")]
        algo: Algorithm,

        #[clap(long, default_value = "20")]
        conc: usize,

        #[clap(long)]
        wordlist: Option<String>,

        #[clap(long, default_value = "")]
        prefix: String,

        #[clap(long, default_value = "")]
        suffix: String,

        #[clap(long, default_value = "1")]
        min_len: usize,

        #[clap(long)]
        common_patterns: bool,
    },
}

#[derive(clap::ValueEnum, Clone, Debug)]
enum Algorithm {
    Sha1,
    Md5,
}

fn hash_string(input: &str, algo: Algorithm) -> String {
    match algo {
        Algorithm::Sha1 => {
            let mut hasher = Sha1::new();
            hasher.update(input);
            format!("{:x}", hasher.finalize())
        }
        Algorithm::Md5 => {
            let digest = md5_compute(input);
            format!("{:x}", digest)
        }
    }
}

fn try_wordlist(target: &str, algo: Algorithm, wordlist_path: &str, verbose: bool) -> Option<String> {
    let file = match File::open(wordlist_path) {
        Ok(f) => f,
        Err(e) => {
            eprintln!("Failed to open wordlist {}: {}", wordlist_path, e);
            return None;
        }
    };
    let reader = BufReader::new(file);
    for line in reader.lines() {
        let word = match line {
            Ok(w) => w.trim().to_string(),
            Err(_) => continue,
        };
        if verbose {
            println!("Trying wordlist entry: {}", word);
        }
        if hash_string(&word, algo.clone()) == target {
            return Some(word);
        }
    }
    None
}

fn try_common_patterns(target: &str, algo: Algorithm, verbose: bool) -> Option<String> {
    let common = vec![
        "password", "admin", "123456", "qwerty", "letmein", "welcome",
        "abc123", "password1", "test123", "admin123", "user", "guest",
    ];
    for word in common {
        if verbose {
            println!("Trying common pattern: {}", word);
        }
        if hash_string(word, algo.clone()) == target {
            return Some(word.to_string());
        }
    }
    None
}

fn brute_force_hash(
    target: &str,
    algo: Algorithm,
    charset: &str,
    min_len: usize,
    max_len: usize,
    conc: usize,
    prefix: &str,
    suffix: &str,
    verbose: bool,
) -> Option<String> {
    let start_time = Instant::now();
    let charset: Vec<char> = charset.chars().collect();
    let fixed_len = prefix.len() + suffix.len();
    let total_combinations: u64 = charset.len() as u64 * (max_len - min_len + 1) as u64; // Estimate for progress bar
    let pb = if verbose {
        let pb = ProgressBar::new(total_combinations);
        pb.set_style(
            ProgressStyle::default_bar()
                .template("{spinner:.green} [{elapsed_precise}] [{bar:40.cyan/blue}] {pos}/{len} ({eta})")
                .unwrap(),
        );
        Some(pb)
    } else {
        None
    };
    let pool = ThreadPoolBuilder::new()
        .num_threads(conc)
        .build()
        .expect("Failed to build thread pool");
    let result = pool.install(|| {
        for len in min_len..=max_len {
            let var_len = len.saturating_sub(fixed_len); // Variable part after prefix/suffix
            let total_for_len = charset.len().pow(var_len as u32);
            let indices: Vec<Vec<usize>> = (0..total_for_len)
                .map(|mut n| {
                    let mut idx = vec![0; var_len];
                    for i in (0..var_len).rev() {
                        idx[i] = n % charset.len();
                        n /= charset.len();
                    }
                    idx
                })
                .collect();
            let found = indices.par_iter().find_map_any(|current| {
                let middle: String = current.iter().map(|&i| charset[i]).collect();
                let candidate = format!("{}{}{}", prefix, middle, suffix);
                if let Some(pb) = &pb {
                    pb.inc(1);
                    if verbose {
                        pb.println(format!("Trying: {}", candidate));
                    }
                }
                if hash_string(&candidate, algo.clone()) == target {
                    Some(candidate)
                } else {
                    None
                }
            });
            if let Some(found) = found {
                if let Some(pb) = pb {
                    pb.finish_with_message("Found match!");
                }
                if verbose {
                    println!("Time taken: {:?}", start_time.elapsed());
                }
                return Some(found);
            }
        }
        if let Some(pb) = pb {
            pb.finish_with_message("No match found");
        }
        None
    });
    if verbose && result.is_none() {
        println!("Time taken: {:?}", start_time.elapsed());
    }
    result
}

fn main() {
    let cli = Cli::parse();
    let verbose = !cli.noverbose;
    let start_time = Instant::now();
    if verbose {
        println!("Starting vox-hash at {}", Local::now());
    }
    match cli.command {
        Commands::Enc { algo, str } => {
            let result = hash_string(&str, algo.clone());
            if verbose {
                println!("Input: {}", str);
                println!("Algorithm: {:?}", algo);
                println!("Hash: {}", result);
                println!("Time taken: {:?}", start_time.elapsed());
            } else {
                println!("{}", result);
            }
        }
        Commands::Dec { key, auto, algo, conc, wordlist, prefix, suffix, min_len, common_patterns } => {
            let algo = if auto {
                match key.len() {
                    32 => Algorithm::Md5,
                    40 => Algorithm::Sha1,
                    _ => {
                        eprintln!("Invalid hash length for auto-detection. Use --algo to specify.");
                        std::process::exit(1);
                    }
                } // Auto-detect algo based on hash length
            } else {
                algo
            };
            if verbose {
                println!("Target hash: {}", key);
                println!("Algorithm: {:?}", algo);
                println!("Charset: {}", cli.charset);
                println!("Min length: {}", min_len);
                println!("Max length: {}", cli.max_len);
                println!("Concurrent threads: {}", conc);
                if let Some(ref wl) = wordlist {
                    println!("Wordlist: {}", wl);
                }
                if !prefix.is_empty() {
                    println!("Prefix: {}", prefix);
                }
                if !suffix.is_empty() {
                    println!("Suffix: {}", suffix);
                }
                if common_patterns {
                    println!("Using common patterns: true");
                }
            }
            if common_patterns {
                if let Some(result) = try_common_patterns(&key, algo.clone(), verbose) {
                    println!("Match found: {}", result);
                    if verbose {
                        println!("Time taken: {:?}", start_time.elapsed());
                    }
                    return;
                }
            }
            if let Some(wordlist_path) = wordlist {
                if let Some(result) = try_wordlist(&key, algo.clone(), &wordlist_path, verbose) {
                    println!("Match found: {}", result);
                    if verbose {
                        println!("Time taken: {:?}", start_time.elapsed());
                    }
                    return;
                }
            }
            match brute_force_hash(&key, algo, &cli.charset, min_len, cli.max_len, conc, &prefix, &suffix, verbose) {
                Some(result) => println!("Match found: {}", result),
                None => println!("No match found within constraints"),
            }
        }
    }
    if verbose {
        println!("Completed at {}", Local::now());
    }
}