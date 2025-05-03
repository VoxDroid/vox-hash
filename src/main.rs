use clap::{Parser, Subcommand, ValueEnum};
use sha1::{Digest, Sha1};
use md5::compute as md5_compute;
use indicatif::{ProgressBar, ProgressStyle};
use chrono::Local;
use std::time::Instant;
use rayon::prelude::*;
use rayon::ThreadPoolBuilder;
use std::fs::{File, OpenOptions};
use std::io::{self, BufRead, BufReader, Write};
use serde_json::{json, Value};
use regex::Regex;

#[derive(Parser)]
#[clap(
    name = "vox-hash",
    version = "1.2",
    about = "A CLI tool for SHA1 and MD5 hashing and brute-force hash matching",
    long_about = "vox-hash supports hashing strings with SHA1/MD5 and brute-force matching of hashes using customizable charsets, wordlists, patterns, and rainbow tables. Use --noverbose to reduce output. Supports bulk operations with newline-separated input files."
)]
struct Cli {
    #[clap(subcommand)]
    command: Commands,

    #[clap(long, global = true)]
    noverbose: bool,

    #[clap(long, default_value = "6")]
    max_len: usize,

    #[clap(long, default_value = "alphanumeric")]
    charset_type: CharsetType,

    #[clap(long)]
    charset: Option<String>,
}

#[derive(Subcommand)]
enum Commands {
    #[clap(about = "Hash a single string using SHA1 or MD5")]
    Enc {
        #[clap(long, value_enum)]
        algo: Algorithm,

        #[clap(long)]
        str: String,

        #[clap(long)]
        output: Option<String>,

        #[clap(long)]
        json: bool,
    },
    #[clap(about = "Brute-force match a single hash with constraints, wordlists, or rainbow tables")]
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
        length: Option<usize>,

        #[clap(long)]
        common_patterns: bool,

        #[clap(long)]
        pattern: Option<String>,

        #[clap(long)]
        rainbow_table: Option<String>,

        #[clap(long)]
        output: Option<String>,

        #[clap(long)]
        json: bool,
    },
    #[clap(about = "Hash multiple strings from a file using SHA1 or MD5")]
    BulkEnc {
        #[clap(long, value_enum)]
        algo: Algorithm,

        #[clap(long)]
        input: String,

        #[clap(long)]
        output: Option<String>,

        #[clap(long)]
        json: bool,
    },
    #[clap(about = "Brute-force match multiple hashes from a file with constraints, wordlists, or rainbow tables")]
    BulkDec {
        #[clap(long)]
        input: String,

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
        length: Option<usize>,

        #[clap(long)]
        common_patterns: bool,

        #[clap(long)]
        pattern: Option<String>,

        #[clap(long)]
        rainbow_table: Option<String>,

        #[clap(long)]
        output: Option<String>,

        #[clap(long)]
        json: bool,

        #[clap(long, default_value = "1000")]
        batch_size: usize,

        #[clap(long)]
        only_success: bool,
    },
    #[clap(about = "Generate a rainbow table for a charset and length range")]
    GenerateTable {
        #[clap(long)]
        output: String,

        #[clap(long, default_value = "1")]
        min_len: usize,

        #[clap(long)]
        max_len: usize,

        #[clap(long, value_enum, default_value = "sha1")]
        algo: Algorithm,
    },
    #[clap(about = "Benchmark hashing speed")]
    Benchmark {
        #[clap(long, value_enum, default_value = "sha1")]
        algo: Algorithm,

        #[clap(long, default_value = "1000000")]
        iterations: usize,
    },
}

#[derive(Clone, ValueEnum, Debug)]
enum Algorithm {
    Sha1,
    Md5,
}

#[derive(Clone, ValueEnum, Debug)]
enum CharsetType {
    Alphanumeric,
    Lowercase,
    Uppercase,
    Digits,
    Custom,
}

fn get_charset(charset_type: &CharsetType, custom_charset: &Option<String>) -> String {
    match charset_type {
        CharsetType::Alphanumeric => "abcdefghijklmnopqrstuvwxyz0123456789".to_string(),
        CharsetType::Lowercase => "abcdefghijklmnopqrstuvwxyz".to_string(),
        CharsetType::Uppercase => "ABCDEFGHIJKLMNOPQRSTUVWXYZ".to_string(),
        CharsetType::Digits => "0123456789".to_string(),
        CharsetType::Custom => custom_charset
            .as_ref()
            .map_or("abcdefghijklmnopqrstuvwxyz0123456789".to_string(), |s| s.clone()),
    }
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

fn validate_hash(hash: &str, algo: Algorithm, auto: bool) -> bool {
    let re = Regex::new(r"^[0-9a-fA-F]+$").unwrap();
    if !re.is_match(hash) {
        return false;
    }
    if auto {
        hash.len() == 32 || hash.len() == 40
    } else {
        match algo {
            Algorithm::Sha1 => hash.len() == 40,
            Algorithm::Md5 => hash.len() == 32,
        }
    } // Check hex format and length
}

fn try_wordlist(target: &str, algo: Algorithm, wordlist_path: &str, verbose: bool, conc: usize) -> Option<String> {
    let file = File::open(wordlist_path).map_err(|e| eprintln!("Failed to open wordlist {}: {}", wordlist_path, e)).ok()?;
    let reader = BufReader::new(file);
    let words: Vec<String> = reader.lines().filter_map(|line| line.ok().map(|w| w.trim().to_string())).collect();
    let pool = ThreadPoolBuilder::new().num_threads(conc).build().expect("Failed to build thread pool");
    pool.install(|| {
        words.par_iter().find_map_any(|word| {
            if verbose {
                println!("Trying wordlist entry: {}", word);
            }
            if hash_string(word, algo.clone()) == target {
                Some(word.clone())
            } else {
                None
            }
        })
    })
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

fn try_rainbow_table(target: &str, table_path: &str) -> Option<String> {
    let file = File::open(table_path).map_err(|e| eprintln!("Failed to open rainbow table {}: {}", table_path, e)).ok()?;
    let table: Value = serde_json::from_reader(file).map_err(|e| eprintln!("Failed to parse rainbow table: {}", e)).ok()?;
    table.as_object()?.get(target).and_then(|v| v.as_str().map(String::from))
}

fn generate_rainbow_table(algo: Algorithm, charset: &str, min_len: usize, max_len: usize, output: &str, verbose: bool) -> io::Result<()> {
    let charset: Vec<char> = charset.chars().collect();
    let mut table = serde_json::Map::new();
    let pb = if verbose {
        let total: u64 = (min_len..=max_len).map(|len| charset.len().pow(len as u32) as u64).sum();
        let pb = ProgressBar::new(total);
        pb.set_style(ProgressStyle::default_bar().template("{spinner:.green} [{elapsed_precise}] [{bar:40.cyan/blue}] {pos}/{len} ({eta})").unwrap());
        Some(pb)
    } else {
        None
    };
    for len in min_len..=max_len {
        let total_for_len = charset.len().pow(len as u32);
        let indices: Vec<Vec<usize>> = (0..total_for_len).map(|mut n| {
            let mut idx = vec![0; len];
            for i in (0..len).rev() {
                idx[i] = n % charset.len();
                n /= charset.len();
            }
            idx
        }).collect();
        for current in indices {
            let candidate: String = current.iter().map(|&i| charset[i]).collect();
            let hash = hash_string(&candidate, algo.clone());
            table.insert(hash, json!(candidate));
            if let Some(pb) = &pb {
                pb.inc(1);
            }
        }
    }
    let mut file = OpenOptions::new().write(true).create(true).open(output)?;
    serde_json::to_writer(&mut file, &table)?;
    if let Some(pb) = pb {
        pb.finish_with_message("Table generated");
    }
    Ok(())
}

fn parse_pattern(pattern: &str) -> Option<(String, usize)> {
    let re = Regex::new(r"^\[([a-z0-9]+)\]\{(\d+)\}$").unwrap();
    re.captures(pattern).map(|caps| {
        let charset = caps[1].to_string();
        let len: usize = caps[2].parse().unwrap();
        (charset, len)
    }) // Parse [charset]{length} format
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
    pattern: Option<&str>,
    verbose: bool,
) -> Option<String> {
    let start_time = Instant::now();
    let charset: Vec<char> = charset.chars().collect();
    let fixed_len = prefix.len() + suffix.len();
    let (charset, min_len, max_len) = if let Some(pattern) = pattern {
        let (pat_charset, len) = parse_pattern(pattern)?;
        (pat_charset.chars().collect(), len, len)
    } else {
        (charset, min_len, max_len)
    };
    let total_combinations: u64 = charset.len() as u64 * (max_len - min_len + 1) as u64; // Estimate for progress bar
    let pb = if verbose {
        let pb = ProgressBar::new(total_combinations);
        pb.set_style(ProgressStyle::default_bar().template("{spinner:.green} [{elapsed_precise}] [{bar:40.cyan/blue}] {pos}/{len} ({eta})").unwrap());
        Some(pb)
    } else {
        None
    };
    let pool = ThreadPoolBuilder::new().num_threads(conc).build().expect("Failed to build thread pool");
    let result = pool.install(|| {
        for len in min_len..=max_len {
            let var_len = len.saturating_sub(fixed_len); // Variable part after prefix/suffix
            let total_for_len = charset.len().pow(var_len as u32);
            let indices: Vec<Vec<usize>> = (0..total_for_len).map(|mut n| {
                let mut idx = vec![0; var_len];
                for i in (0..var_len).rev() {
                    idx[i] = n % charset.len();
                    n /= charset.len();
                }
                idx
            }).collect();
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

fn bulk_enc(algo: Algorithm, input_path: &str, output: Option<&str>, json: bool, verbose: bool) -> io::Result<()> {
    let file = File::open(input_path).map_err(|e| io::Error::new(io::ErrorKind::NotFound, format!("Failed to open input {}: {}", input_path, e)))?;
    let reader = BufReader::new(file);
    let lines: Vec<String> = reader.lines().filter_map(|line| line.ok().filter(|s| !s.trim().is_empty())).collect();
    let total = lines.len() as u64;
    let pb = if verbose {
        let pb = ProgressBar::new(total);
        pb.set_style(ProgressStyle::default_bar().template("{spinner:.green} [{elapsed_precise}] [{bar:40.cyan/blue}] {pos}/{len} ({eta})").unwrap());
        Some(pb)
    } else {
        None
    };
    let results: Vec<(String, String)> = lines.into_iter().map(|line| {
        let hash = hash_string(&line, algo.clone());
        if let Some(pb) = &pb {
            pb.inc(1);
        }
        (line, hash)
    }).collect();
    let output_str = if json {
        json!(results.iter().map(|(input, hash)| json!({ "input": input, "hash": hash })).collect::<Vec<_>>()).to_string()
    } else {
        results.iter().map(|(_, hash)| hash.as_str()).collect::<Vec<_>>().join("\n")
    };
    if let Some(path) = output {
        let mut file = OpenOptions::new().write(true).create(true).open(path)?;
        writeln!(file, "{}", output_str)?;
    }
    if verbose {
        for (input, hash) in results {
            println!("Input: {}, Hash: {}", input, hash);
        }
    } else if output.is_none() {
        println!("{}", output_str);
    }
    if let Some(pb) = pb {
        pb.finish_with_message("Processing complete");
    }
    Ok(())
}

fn bulk_dec(
    input_path: &str,
    auto: bool,
    algo: Algorithm,
    conc: usize,
    wordlist: Option<&str>,
    prefix: &str,
    suffix: &str,
    min_len: usize,
    max_len: usize,
    common_patterns: bool,
    pattern: Option<&str>,
    rainbow_table: Option<&str>,
    output: Option<&str>,
    json: bool,
    batch_size: usize,
    only_success: bool,
    charset: &str,
    verbose: bool,
) -> io::Result<()> {
    let file = File::open(input_path).map_err(|e| io::Error::new(io::ErrorKind::NotFound, format!("Failed to open input {}: {}", input_path, e)))?;
    let reader = BufReader::new(file);
    let hashes: Vec<String> = reader.lines().filter_map(|line| line.ok().filter(|s| !s.trim().is_empty())).collect();
    let total = hashes.len() as u64;
    let pb = if verbose {
        let pb = ProgressBar::new(total);
        pb.set_style(ProgressStyle::default_bar().template("{spinner:.green} [{elapsed_precise}] [{bar:40.cyan/blue}] {pos}/{len} ({eta})").unwrap());
        Some(pb)
    } else {
        None
    };
    let pool = ThreadPoolBuilder::new().num_threads(conc).build().expect("Failed to build thread pool");
    let results: Vec<(String, String)> = pool.install(|| {
        hashes.par_chunks(batch_size).flat_map(|batch| {
            batch.par_iter().filter_map(|hash| {
                if !validate_hash(hash, algo.clone(), auto) {
                    if verbose {
                        eprintln!("Skipping invalid hash: {}", hash);
                    }
                    return None;
                }
                let mut result = None;
                if let Some(table_path) = rainbow_table {
                    result = try_rainbow_table(hash, table_path);
                }
                if result.is_none() && common_patterns {
                    result = try_common_patterns(hash, algo.clone(), verbose);
                }
                if result.is_none() {
                    if let Some(wordlist_path) = wordlist {
                        result = try_wordlist(hash, algo.clone(), wordlist_path, verbose, conc);
                    }
                }
                if result.is_none() {
                    result = brute_force_hash(hash, algo.clone(), charset, min_len, max_len, conc, prefix, suffix, pattern, verbose);
                }
                if let Some(pb) = &pb {
                    pb.inc(1);
                }
                Some((hash.clone(), result.unwrap_or_else(|| "No match found".to_string())))
            }).collect::<Vec<_>>()
        }).collect()
    });
    let output_str = if json {
        json!(results.iter().map(|(hash, result)| json!({ "hash": hash, "result": result })).collect::<Vec<_>>()).to_string()
    } else {
        results.iter().filter(|(_, result)| !only_success || result != "No match found").map(|(_, result)| result.as_str()).collect::<Vec<_>>().join("\n")
    };
    if let Some(path) = output {
        let mut file = OpenOptions::new().write(true).create(true).open(path)?;
        writeln!(file, "{}", output_str)?;
    }
    if verbose {
        let matches = results.iter().filter(|(_, result)| result != "No match found").count();
        for (hash, result) in &results {
            println!("Hash: {}, Result: {}", hash, result);
        }
        println!("Found {}/{} matches", matches, results.len());
    } else if output.is_none() {
        println!("{}", output_str);
    }
    if let Some(pb) = pb {
        pb.finish_with_message("Processing complete");
    }
    Ok(())
}

fn benchmark(algo: Algorithm, iterations: usize) -> f64 {
    let start = Instant::now();
    for _ in 0..iterations {
        hash_string("test", algo.clone());
    }
    let elapsed = start.elapsed().as_secs_f64();
    (iterations as f64) / elapsed // Hashes per second
}

fn write_output(result: &str, output: Option<&str>, json: bool, verbose: bool) -> io::Result<()> {
    let output_str = if json {
        json!({ "result": result }).to_string()
    } else {
        result.to_string()
    };
    if let Some(path) = output {
        let mut file = OpenOptions::new().write(true).create(true).open(path)?;
        writeln!(file, "{}", output_str)?;
    } else if !verbose {
        println!("{}", output_str);
    }
    Ok(())
}

fn main() {
    let cli = Cli::parse();
    let verbose = !cli.noverbose;
    let charset = get_charset(&cli.charset_type, &cli.charset);
    let start_time = Instant::now();
    if verbose {
        println!("Starting vox-hash at {}", Local::now());
    }
    match cli.command {
        Commands::Enc { algo, str, output, json } => {
            let result = hash_string(&str, algo.clone());
            if verbose {
                println!("Input: {}", str);
                println!("Algorithm: {:?}", algo);
                println!("Hash: {}", result);
                println!("Time taken: {:?}", start_time.elapsed());
            }
            write_output(&result, output.as_deref(), json, verbose).expect("Failed to write output");
        }
        Commands::Dec { key, auto, algo, conc, wordlist, prefix, suffix, min_len, length, common_patterns, pattern, rainbow_table, output, json } => {
            if min_len > cli.max_len {
                eprintln!("Error: --min-len must be <= --max-len");
                std::process::exit(1);
            }
            if conc == 0 {
                eprintln!("Error: --conc must be > 0");
                std::process::exit(1);
            }
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
            if !validate_hash(&key, algo.clone(), auto) {
                eprintln!("Invalid hash format or length");
                std::process::exit(1);
            }
            let min_len = length.unwrap_or(min_len);
            let max_len = length.unwrap_or(cli.max_len);
            if verbose {
                println!("Target hash: {}", key);
                println!("Algorithm: {:?}", algo);
                println!("Charset: {}", charset);
                println!("Min length: {}", min_len);
                println!("Max length: {}", max_len);
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
                if let Some(ref pat) = pattern {
                    println!("Pattern: {}", pat);
                }
                if let Some(ref rt) = rainbow_table {
                    println!("Rainbow table: {}", rt);
                }
                if common_patterns {
                    println!("Using common patterns: true");
                }
            }
            let mut result = None;
            if let Some(table_path) = rainbow_table {
                result = try_rainbow_table(&key, &table_path);
            }
            if result.is_none() && common_patterns {
                result = try_common_patterns(&key, algo.clone(), verbose);
            }
            if result.is_none() {
                if let Some(wordlist_path) = wordlist {
                    result = try_wordlist(&key, algo.clone(), &wordlist_path, verbose, conc);
                }
            }
            if result.is_none() {
                result = brute_force_hash(&key, algo, &charset, min_len, max_len, conc, &prefix, &suffix, pattern.as_deref(), verbose);
            }
            match result {
                Some(res) => {
                    if verbose {
                        println!("Match found: {}", res);
                        println!("Time taken: {:?}", start_time.elapsed());
                    }
                    write_output(&res, output.as_deref(), json, verbose).expect("Failed to write output");
                }
                None => println!("No match found within constraints"),
            }
        }
        Commands::BulkEnc { algo, input, output, json } => {
            bulk_enc(algo, &input, output.as_deref(), json, verbose).expect("Failed to process bulk encryption");
        }
        Commands::BulkDec { input, auto, algo, conc, wordlist, prefix, suffix, min_len, length, common_patterns, pattern, rainbow_table, output, json, batch_size, only_success } => {
            if min_len > cli.max_len {
                eprintln!("Error: --min-len must be <= --max-len");
                std::process::exit(1);
            }
            if conc == 0 {
                eprintln!("Error: --conc must be > 0");
                std::process::exit(1);
            }
            if batch_size == 0 {
                eprintln!("Error: --batch-size must be > 0");
                std::process::exit(1);
            }
            let min_len = length.unwrap_or(min_len);
            let max_len = length.unwrap_or(cli.max_len);
            if verbose {
                println!("Input file: {}", input);
                println!("Algorithm: {:?}", algo);
                println!("Charset: {}", charset);
                println!("Min length: {}", min_len);
                println!("Max length: {}", max_len);
                println!("Concurrent threads: {}", conc);
                println!("Batch size: {}", batch_size);
                if let Some(ref wl) = wordlist {
                    println!("Wordlist: {}", wl);
                }
                if !prefix.is_empty() {
                    println!("Prefix: {}", prefix);
                }
                if !suffix.is_empty() {
                    println!("Suffix: {}", suffix);
                }
                if let Some(ref pat) = pattern {
                    println!("Pattern: {}", pat);
                }
                if let Some(ref rt) = rainbow_table {
                    println!("Rainbow table: {}", rt);
                }
                if common_patterns {
                    println!("Using common patterns: true");
                }
                if only_success {
                    println!("Only output successful matches: true");
                }
            }
            bulk_dec(
                &input,
                auto,
                algo,
                conc,
                wordlist.as_deref(),
                &prefix,
                &suffix,
                min_len,
                max_len,
                common_patterns,
                pattern.as_deref(),
                rainbow_table.as_deref(),
                output.as_deref(),
                json,
                batch_size,
                only_success,
                &charset,
                verbose,
            ).expect("Failed to process bulk decryption");
        }
        Commands::GenerateTable { output, min_len, max_len, algo } => {
            if min_len > max_len {
                eprintln!("Error: --min-len must be <= --max-len");
                std::process::exit(1);
            }
            generate_rainbow_table(algo, &charset, min_len, max_len, &output, verbose).expect("Failed to generate rainbow table");
            if verbose {
                println!("Rainbow table saved to {}", output);
            }
        }
        Commands::Benchmark { algo, iterations } => {
            let hashes_per_sec = benchmark(algo, iterations);
            println!("Benchmark: {:.2} hashes/second", hashes_per_sec);
        }
    }
    if verbose {
        println!("Completed at {}", Local::now());
    }
}