use std::{collections::HashMap, path::Path};

use aes_gcm::{aead::Aead, Aes256Gcm, Key, KeyInit};
use anyhow::{format_err, Result};

use crate::{utils::hex_to_bytes, PASSWORD_DB_PATH};

pub fn decrypt_data(data: &Vec<u8>, key: &String) -> Result<String> {
    let key_bytes = hex_to_bytes(key)?;
    let key_bytes = key_bytes.as_slice();

    let key: &Key<Aes256Gcm> = key_bytes.try_into()?;
    let cipher = Aes256Gcm::new(&key);

    let nonce = &data[0..12];
    let data = &data[12..];

    match cipher.decrypt(nonce.try_into()?, data) {
        Ok(plaintext) => Ok(String::from_utf8(plaintext)?),
        Err(e) => Err(format_err!("Error decrypting data: {}", e)),
    }
}

pub fn read_password_database(key: &String) -> Result<HashMap<String, String>> {
    if !Path::new(PASSWORD_DB_PATH).exists() {
        return Ok(HashMap::new());
    }

    let file_data = std::fs::read(PASSWORD_DB_PATH).unwrap();
    let decrypted_data = decrypt_data(&file_data, key)?;
    let password_db: HashMap<String, String> = serde_json::from_str(&decrypted_data).unwrap();

    return Ok(password_db);
}
