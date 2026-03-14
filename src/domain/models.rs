use serde::Serialize;

#[derive(Serialize, Debug)]
pub struct HashResult {
    pub input: String,
    pub hash: String,
}

#[derive(Serialize, Debug)]
pub struct DecryptionResult {
    pub hash: String,
    pub result: String,
}

#[derive(Serialize, Debug)]
pub struct BulkDecryptionResult {
    pub hash: String,
    pub result: String,
}
