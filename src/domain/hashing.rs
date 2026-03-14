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

    pub fn detect_from_hash(hash: &str) -> Option<Self> {
        let hash = hash.trim().to_lowercase();
        if Algorithm::Sha1.get_implementation().validate_format(&hash) {
            Some(Algorithm::Sha1)
        } else if Algorithm::Md5.get_implementation().validate_format(&hash) {
            Some(Algorithm::Md5)
        } else {
            None
        }
    }
}

pub fn hash_string(input: &str, algo: Algorithm) -> String {
    algo.get_implementation().hash(input)
}

pub fn validate_hash(hash: &str, algo: Algorithm, auto: bool) -> bool {
    if auto {
        Algorithm::detect_from_hash(hash).is_some()
    } else {
        algo.get_implementation().validate_format(hash)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_sha1_hash() {
        let algo = Algorithm::Sha1;
        assert_eq!(
            hash_string("test", algo),
            "a94a8fe5ccb19ba61c4c0873d391e987982fbbd3"
        );
        assert_eq!(
            hash_string("", algo),
            "da39a3ee5e6b4b0d3255bfef95601890afd80709"
        );
    }

    #[test]
    fn test_md5_hash() {
        let algo = Algorithm::Md5;
        assert_eq!(
            hash_string("test", algo),
            "098f6bcd4621d373cade4e832627b4f6"
        );
        assert_eq!(hash_string("", algo), "d41d8cd98f00b204e9800998ecf8427e");
    }

    #[test]
    fn test_detect_from_hash() {
        let sha1_hash = "a94a8fe5ccb19ba61c4c0873d391e987982fbbd3";
        let md5_hash = "098f6bcd4621d373cade4e832627b4f6";
        let invalid_hash = "abc";

        assert_eq!(
            Algorithm::detect_from_hash(sha1_hash),
            Some(Algorithm::Sha1)
        );
        assert_eq!(Algorithm::detect_from_hash(md5_hash), Some(Algorithm::Md5));
        assert_eq!(Algorithm::detect_from_hash(invalid_hash), None);
    }

    #[test]
    fn test_validate_hash() {
        let sha1_hash = "a94a8fe5ccb19ba61c4c0873d391e987982fbbd3";
        let md5_hash = "098f6bcd4621d373cade4e832627b4f6";

        assert!(validate_hash(sha1_hash, Algorithm::Sha1, false));
        assert!(validate_hash(md5_hash, Algorithm::Md5, false));
        assert!(!validate_hash(sha1_hash, Algorithm::Md5, false));
        assert!(validate_hash(sha1_hash, Algorithm::Md5, true));
    }
}
