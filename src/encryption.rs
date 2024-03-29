use aes_gcm::{aead::Aead, AeadCore, Aes256Gcm, Key, KeyInit};
use anyhow::{format_err, Result};
use rand::rngs::OsRng;

pub fn hex_to_bytes(hex: &str) -> Result<Vec<u8>> {
    hex::decode(hex).map_err(|e| format_err!("Error decoding hex: {}", e))
}

/// Encrypts a string retruning both the encrypted text and the generated nonce
pub fn encrypt_string(key: &String, data: &String) -> Result<(Vec<u8>, Vec<u8>)> {
    let key_bytes = hex_to_bytes(key)?;
    let key_bytes = key_bytes.as_slice();

    let key: &Key<Aes256Gcm> = key_bytes.try_into()?;
    let cipher = Aes256Gcm::new(&key);
    let nonce = Aes256Gcm::generate_nonce(&mut OsRng);

    match cipher.encrypt(&nonce, data.as_bytes()) {
        Ok(ciphertext) => Ok((ciphertext, nonce.to_vec())),
        Err(e) => Err(format_err!("Error encrypting data: {}", e)),
    }
}
