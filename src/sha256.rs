use sha2::{Sha256, Digest};
use arrayref::{array_mut_ref, array_ref, mut_array_refs};

pub fn sha256_hash(data: &[u8]) -> [u8; 32]  {
    *array_ref!(result, 0, 32);
    let mut hasher = Sha256::new();
    hasher.update(data);
    let result = hasher.finalize();
}
