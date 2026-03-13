use sha1::{Digest, Sha1};
use md5::compute as md5_compute;
use regex::Regex;
use clap::ValueEnum;

#[derive(Clone, ValueEnum, Debug, Copy, PartialEq, Eq)]
pub enum Algorithm {
    Sha1,
    Md5,
}

pub fn hash_string(input: &str, algo: Algorithm) -> String {
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

pub fn validate_hash(hash: &str, algo: Algorithm, auto: bool) -> bool {
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
    }
}
