use sha2::{Sha256, Digest};

pub fn encrypt(raw: &str) -> String {
    let mut hasher = Sha256::new();
    hasher.update(raw);
    format!("{:x}", hasher.finalize())
}