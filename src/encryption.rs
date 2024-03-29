use aes_gcm::{aead::Aead, AeadCore, Aes256Gcm, Key, KeyInit};
use anyhow::{format_err, Result};
use rand::rngs::OsRng;
use std::collections::HashMap;

use crate::{utils::slice_to_fixed_array, PASSWORD_DB_PATH};

/// Encrypts a string retruning both the encrypted text and the generated nonce
pub fn encrypt_string(key: &String, data: &String) -> Result<(Vec<u8>, Vec<u8>)> {
    let key: [u8; 32] = slice_to_fixed_array(&key.as_bytes());
    let key = Key::<Aes256Gcm>::from_slice(&key);
    let cipher = Aes256Gcm::new(&key);
    let nonce = Aes256Gcm::generate_nonce(&mut OsRng);

    match cipher.encrypt(&nonce, data.as_bytes()) {
        Ok(ciphertext) => Ok((ciphertext, nonce.to_vec())),
        Err(e) => Err(format_err!("Error encrypting data: {}", e)),
    }
}

/// Encrypts and writes the password database to disk
pub fn write_password_database(
    password_db: HashMap<String, String>,
    master_password: String,
) -> Result<()> {
    let serialized = serde_json::to_string(&password_db).unwrap();
    let (encrypted, mut nonce) = encrypt_string(&master_password, &serialized)?;

    nonce.extend(encrypted);

    std::fs::write(PASSWORD_DB_PATH, nonce).unwrap();

    return Ok(());
}
