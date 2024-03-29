use std::{collections::HashMap, path::Path};

use aes_gcm::{aead::Aead, Aes256Gcm, Key, KeyInit};
use anyhow::{format_err, Result};

use crate::{
    hashing::hash_key,
    utils::{check_key, hex_to_bytes, prompt_for_master_key},
    PASSWORD_DB_PATH,
};

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

pub fn read_password_database() -> Result<(HashMap<String, String>, String)> {
    let (mut master_hash, new_master_key) = check_key();
    if new_master_key == false {
        let entered_master_key_hash =
            hash_key(&prompt_for_master_key("Please enter master password: "));

        if master_hash != entered_master_key_hash {
            println!("Incorrect master password");
        }

        master_hash = entered_master_key_hash;
    }

    if !Path::new(PASSWORD_DB_PATH).exists() {
        return Ok((HashMap::new(), master_hash));
    }

    let file_data = std::fs::read(PASSWORD_DB_PATH).unwrap();
    let decrypted_data = decrypt_data(&file_data, &master_hash)?;
    let password_db: HashMap<String, String> = serde_json::from_str(&decrypted_data).unwrap();

    return Ok((password_db, master_hash));
}
