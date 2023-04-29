use sha2::{Digest, Sha256};

#[derive(Debug)]
pub struct Block {
    pub index: u32,
    pub timestamp: u128,
    pub data: Vec<u8>,
    pub previous_hash: [u8; 32],
    pub hash: [u8; 32],
    pub nonce: u64,
}

impl Block {
    pub fn new(index: u32, timestamp: u128, data: Vec<u8>, previous_hash: [u8; 32], nonce: u64) -> Self {
        let mut hasher = Sha256::new();
        hasher.update(index.to_be_bytes());
        hasher.update(timestamp.to_be_bytes());
        hasher.update(&data);
        hasher.update(&previous_hash);
        hasher.update(&nonce.to_be_bytes());
        let hash = hasher.finalize();

        Self {
            index,
            timestamp,
            data,
            previous_hash,
            hash: hash.into(),
            nonce,
        }
    }
}
