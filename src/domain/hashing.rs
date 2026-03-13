use clap::ValueEnum;
use md5::compute as md5_compute;
use regex::Regex;
use sha1::{Digest, Sha1};

pub trait HashAlgorithm: Send + Sync {
    fn hash(&self, input: &str) -> String;
    fn expected_length(&self) -> usize;
    fn validate_format(&self, hash: &str) -> bool {
        let hash = hash.to_lowercase();
        let re = Regex::new(r"^[0-9a-f]+$").unwrap();
        re.is_match(&hash) && hash.len() == self.expected_length()
    }
}

pub struct Sha1Algo;
impl HashAlgorithm for Sha1Algo {
    fn hash(&self, input: &str) -> String {
        let mut hasher = Sha1::new();
        hasher.update(input);
        format!("{:x}", hasher.finalize())
    }
    fn expected_length(&self) -> usize {
        40
    }
}

pub struct Md5Algo;
impl HashAlgorithm for Md5Algo {
    fn hash(&self, input: &str) -> String {
        let digest = md5_compute(input);
        format!("{:x}", digest)
    }
    fn expected_length(&self) -> usize {
        32
    }
}

#[derive(Clone, ValueEnum, Debug, Copy, PartialEq, Eq)]
pub enum Algorithm {
    Sha1,
    Md5,
}

impl Algorithm {
    pub fn get_implementation(&self) -> Box<dyn HashAlgorithm> {
        match self {
            Algorithm::Sha1 => Box::new(Sha1Algo),
            Algorithm::Md5 => Box::new(Md5Algo),
        }
    }
}

pub fn hash_string(input: &str, algo: Algorithm) -> String {
    algo.get_implementation().hash(input)
}

pub fn validate_hash(hash: &str, algo: Algorithm, auto: bool) -> bool {
    let hash = hash.trim().to_lowercase();
    if auto {
        Algorithm::Sha1.get_implementation().validate_format(&hash)
            || Algorithm::Md5.get_implementation().validate_format(&hash)
    } else {
        algo.get_implementation().validate_format(&hash)
    }
}
