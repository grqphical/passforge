use sha2::{Digest, Sha256};

pub fn hash_key(key: &String) -> String {
    let mut hasher = Sha256::new();
    hasher.update(key.as_bytes());
    let result = hasher.finalize();
    format!("{:x}", result)
}
