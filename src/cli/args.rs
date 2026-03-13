use clap::{Parser, Subcommand, ArgAction};
use crate::domain::hashing::Algorithm;
use crate::domain::candidate_generation::CharsetType;

#[derive(Parser, Debug)]
#[clap(
    name = "vox-hash",
    version = "1.3",
    about = "A CLI tool for SHA1 and MD5 hashing and brute-force hash matching",
    long_about = "vox-hash is a powerful command-line tool for hashing strings with SHA1 or MD5 and performing brute-force decryption of hashes. It supports single and bulk operations, customizable charsets, wordlists, patterns, and rainbow tables. Use --noverbose to reduce output. Ideal for security testing and hash analysis.",
    after_help = "EXAMPLES:\n  vox-hash enc --algo sha1 --str 'test'              # Hash 'test' with SHA1\n  vox-hash dec --key 5baa61e4... --auto --wordlist words.txt  # Decrypt hash using wordlist\n  vox-hash bulk-enc --algo md5 --input strings.txt  # Bulk hash strings from file\n  vox-hash bulk-dec --input hashes.txt --auto       # Bulk decrypt hashes"
)]
pub struct Cli {
    #[clap(subcommand)]
    pub command: Commands,

    #[clap(long, global = true, help = "Disable verbose output")]
    pub noverbose: bool,

    #[clap(long, global = true, default_value = "6", help = "Maximum length for brute-force (default: 6)", value_parser = clap::value_parser!(u32).range(1..))]
    pub max_len: u32,

    #[clap(long, global = true, default_value = "alphanumeric", help = "Charset type: alphanumeric, lowercase, uppercase, digits, or custom (default: alphanumeric)")]
    pub charset_type: CharsetType,

    #[clap(long, global = true, help = "Custom charset string (overrides charset_type if provided)")]
    pub charset: Option<String>,
}

#[derive(Subcommand, Debug)]
pub enum Commands {
    #[clap(
        about = "Hash a single string using SHA1 or MD5",
        after_help = "Hashes the provided string and outputs the result. Use --json for JSON format."
    )]
    Enc {
        #[clap(long, value_enum, default_value = "sha1", help = "Hashing algorithm: sha1 or md5 (default: sha1)")]
        algo: Algorithm,

        #[clap(long, help = "String to hash", required = true)]
        str: String,

        #[clap(long, help = "Output file path (optional)")]
        output: Option<String>,

        #[clap(long, help = "Output result in JSON format")]
        json: bool,
    },
    #[clap(
        about = "Brute-force match a single hash with constraints, wordlists, or rainbow tables",
        after_help = "Attempts to find the plaintext for a given hash. Use --auto to detect algorithm, or specify --algo. Combine with --wordlist, --pattern, or --rainbow-table for efficiency.\nGlobal options like --max-len, --charset-type, and --charset can also be used."
    )]
    Dec {
        #[clap(long, help = "Hash to decrypt", required = true)]
        key: String,

        #[clap(long, help = "Automatically detect algorithm based on hash length (MD5=32, SHA1=40)")]
        auto: bool,

        #[clap(long, value_enum, default_value = "sha1", help = "Hashing algorithm: sha1 or md5")]
        algo: Algorithm,

        #[clap(long, default_value = "20", help = "Number of concurrent threads (default: 20)", value_parser = clap::value_parser!(u32).range(1..))]
        conc: u32,

        #[clap(long, help = "Path to wordlist file for decryption")]
        wordlist: Option<String>,

        #[clap(long, default_value = "", help = "Prefix to append to candidates")]
        prefix: String,

        #[clap(long, default_value = "", help = "Suffix to append to candidates")]
        suffix: String,

        #[clap(long, default_value = "1", help = "Minimum length of candidates (default: 1)", value_parser = clap::value_parser!(u32).range(1..))]
        min_len: u32,

        #[clap(long, help = "Fixed length of candidates (overrides min-len and max-len if provided)", value_parser = clap::value_parser!(u32).range(1..))]
        length: Option<u32>,

        #[clap(long, action = ArgAction::Set, default_value = "true", help = "Try common patterns (e.g., 'password', '123456') (default: true)")]
        common_patterns: bool,

        #[clap(long, help = "Pattern in [charset]{length} format (e.g., [a-z]{4})")]
        pattern: Option<String>,

        #[clap(long, help = "Path to rainbow table file")]
        rainbow_table: Option<String>,

        #[clap(long, help = "Output file path (optional)")]
        output: Option<String>,

        #[clap(long, help = "Output result in JSON format")]
        json: bool,
    },
    #[clap(
        about = "Hash multiple strings from a file using SHA1 or MD5",
        after_help = "Reads strings from a file (one per line) and hashes them. Use --json for JSON output."
    )]
    BulkEnc {
        #[clap(long, value_enum, default_value = "sha1", help = "Hashing algorithm: sha1 or md5")]
        algo: Algorithm,

        #[clap(long, help = "Input file path containing strings to hash", required = true)]
        input: String,

        #[clap(long, help = "Output file path (optional)")]
        output: Option<String>,

        #[clap(long, help = "Output result in JSON format")]
        json: bool,
    },
    #[clap(
        about = "Brute-force match multiple hashes from a file with constraints, wordlists, or rainbow tables",
        after_help = "Reads hashes from a file (one per line) and attempts to decrypt them. Use --auto for algorithm detection, and --only-success to filter results.\nGlobal options like --max-len, --charset-type, and --charset can also be used."
    )]
    BulkDec {
        #[clap(long, help = "Input file path containing hashes to decrypt", required = true)]
        input: String,

        #[clap(long, help = "Automatically detect algorithm based on hash length (MD5=32, SHA1=40)")]
        auto: bool,

        #[clap(long, value_enum, default_value = "sha1", help = "Hashing algorithm: sha1 or md5")]
        algo: Algorithm,

        #[clap(long, default_value = "20", help = "Number of concurrent threads (default: 20)", value_parser = clap::value_parser!(u32).range(1..))]
        conc: u32,

        #[clap(long, help = "Path to wordlist file for decryption")]
        wordlist: Option<String>,

        #[clap(long, default_value = "", help = "Prefix to append to candidates")]
        prefix: String,

        #[clap(long, default_value = "", help = "Suffix to append to candidates")]
        suffix: String,

        #[clap(long, default_value = "1", help = "Minimum length of candidates (default: 1)", value_parser = clap::value_parser!(u32).range(1..))]
        min_len: u32,

        #[clap(long, help = "Fixed length of candidates (overrides min-len and max-len if provided)", value_parser = clap::value_parser!(u32).range(1..))]
        length: Option<u32>,

        #[clap(long, action = ArgAction::Set, default_value = "true", help = "Try common patterns (e.g., 'password', '123456') (default: true)")]
        common_patterns: bool,

        #[clap(long, help = "Pattern in [charset]{length} format (e.g., [a-z]{4})")]
        pattern: Option<String>,

        #[clap(long, help = "Path to rainbow table file")]
        rainbow_table: Option<String>,

        #[clap(long, help = "Output file path (optional)")]
        output: Option<String>,

        #[clap(long, help = "Output result in JSON format")]
        json: bool,

        #[clap(long, default_value = "1000", help = "Batch size for parallel processing (default: 1000)", value_parser = clap::value_parser!(u32).range(1..))]
        batch_size: u32,

        #[clap(long, help = "Output only successful matches")]
        only_success: bool,
    },
    #[clap(
        about = "Generate a rainbow table for a charset and length range",
        after_help = "Creates a JSON rainbow table file for precomputed hashes. Useful for speeding up decryption."
    )]
    GenerateTable {
        #[clap(long, help = "Output file path for the rainbow table")]
        output: String,

        #[clap(long, default_value = "1", help = "Minimum length of candidates (default: 1)", value_parser = clap::value_parser!(u32).range(1..))]
        min_len: u32,

        #[clap(long, help = "Maximum length of candidates", value_parser = clap::value_parser!(u32).range(1..))]
        max_len: u32,

        #[clap(long, value_enum, default_value = "sha1", help = "Hashing algorithm: sha1 or md5")]
        algo: Algorithm,
    },
    #[clap(
        about = "Benchmark hashing speed",
        after_help = "Measures hashes per second for the specified algorithm and iteration count."
    )]
    Benchmark {
        #[clap(long, value_enum, default_value = "sha1", help = "Hashing algorithm: sha1 or md5")]
        algo: Algorithm,

        #[clap(long, default_value = "1000000", help = "Number of iterations (default: 1000000)", value_parser = clap::value_parser!(u32).range(1..))]
        iterations: u32,
    },
}
