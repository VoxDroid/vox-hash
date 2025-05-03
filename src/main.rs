use clap::{Parser, Subcommand};
use sha1::{Digest, Sha1};
use md5::compute as md5_compute;
use indicatif::{ProgressBar, ProgressStyle};
use chrono::Local;
use std::time::Instant;
use rayon::prelude::*;
use rayon::ThreadPoolBuilder;

#[derive(Parser)]
#[clap(name = "vox-hash", version = "1.0", about = "A CLI tool for SHA1 and MD5 hashing and brute-force hash matching")]
struct Cli {
    #[clap(subcommand)]
    command: Commands,

    /// Disable verbose output
    #[clap(long, global = true)]
    noverbose: bool,

    /// Set maximum length for brute-force (default: 6)
    #[clap(long, default_value = "6")]
    max_len: usize,

    /// Set charset for brute-force (default: alphanumeric)
    #[clap(long, default_value = "abcdefghijklmnopqrstuvwxyz0123456789")]
    charset: String,
}

#[derive(Subcommand)]
enum Commands {
    /// Hash a string using SHA1 or MD5
    Enc {
        /// Hash algorithm: sha1 or md5
        #[clap(long, value_enum)]
        algo: Algorithm,

        /// String to hash
        #[clap(long)]
        str: String,
    },
    /// Brute-force match a hash
    Dec {
        /// Target hash to match
        #[clap(long)]
        key: String,

        /// Auto-detect algorithm (sha1 or md5) based on hash length
        #[clap(long)]
        auto: bool,

        /// Specify algorithm (sha1 or md5) if not using auto
        #[clap(long, value_enum, default_value = "sha1")]
        algo: Algorithm,

        /// Number of concurrent threads (default: 20)
        #[clap(long, default_value = "20")]
        conc: usize,
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

fn brute_force_hash(target: &str, algo: Algorithm, charset: &str, max_len: usize, conc: usize, verbose: bool) -> Option<String> {
    let start_time = Instant::now();
    let charset: Vec<char> = charset.chars().collect();
    let total_combinations: u64 = charset.len() as u64 * max_len as u64;
    let pb = ProgressBar::new(total_combinations);
    pb.set_style(
        ProgressStyle::default_bar()
            .template("{spinner:.green} [{elapsed_precise}] [{bar:40.cyan/blue}] {pos}/{len} ({eta})")
            .unwrap(),
    );

    // Configure Rayon thread pool
    let pool = ThreadPoolBuilder::new()
        .num_threads(conc)
        .build()
        .expect("Failed to build thread pool");

    let result = pool.install(|| {
        for len in 1..=max_len {
            let total_for_len = charset.len().pow(len as u32);
            let indices: Vec<Vec<usize>> = (0..total_for_len)
                .map(|mut n| {
                    let mut idx = vec![0; len];
                    for i in (0..len).rev() {
                        idx[i] = n % charset.len();
                        n /= charset.len();
                    }
                    idx
                })
                .collect();

            let found = indices.par_iter().find_map_any(|current| {
                let candidate: String = current.iter().map(|&i| charset[i]).collect();
                pb.inc(1);
                if verbose {
                    pb.println(format!("Trying: {}", candidate));
                }
                if hash_string(&candidate, algo.clone()) == target {
                    Some(candidate)
                } else {
                    None
                }
            });

            if let Some(found) = found {
                pb.finish_with_message("Found match!");
                if verbose {
                    println!("Time taken: {:?}", start_time.elapsed());
                }
                return Some(found);
            }
        }
        pb.finish_with_message("No match found");
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
        Commands::Dec { key, auto, algo, conc } => {
            let algo = if auto {
                match key.len() {
                    32 => Algorithm::Md5,
                    40 => Algorithm::Sha1,
                    _ => {
                        eprintln!("Invalid hash length for auto-detection. Use --algo to specify.");
                        std::process::exit(1);
                    }
                }
            } else {
                algo
            };

            if verbose {
                println!("Target hash: {}", key);
                println!("Algorithm: {:?}", algo);
                println!("Charset: {}", cli.charset);
                println!("Max length: {}", cli.max_len);
                println!("Concurrent threads: {}", conc);
            }

            match brute_force_hash(&key, algo, &cli.charset, cli.max_len, conc, verbose) {
                Some(result) => println!("Match found: {}", result),
                None => println!("No match found within constraints"),
            }
        }
    }

    if verbose {
        println!("Completed at {}", Local::now());
    }
}