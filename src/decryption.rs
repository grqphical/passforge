use aes_gcm::{aead::Aead, Aes256Gcm, Key, KeyInit};
use anyhow::{format_err, Result};
use std::{collections::HashMap, path::Path};

use crate::{
    hashing::hash_key,
    utils::{check_key, prompt_for_master_key, slice_to_fixed_array},
    PASSWORD_DB_PATH,
};

pub fn decrypt_data(data: &Vec<u8>, key: &String) -> Result<String> {
    let key: [u8; 32] = slice_to_fixed_array(&key.as_bytes());
    let key = Key::<Aes256Gcm>::from_slice(&key);
    let cipher = Aes256Gcm::new(&key);

    let nonce = &data[0..12];
    let data = &data[12..];

    match cipher.decrypt(nonce.try_into()?, data) {
        Ok(plaintext) => Ok(String::from_utf8(plaintext)?),
        Err(e) => Err(format_err!("Error decrypting data: {}", e)),
    }
}

pub fn read_password_database() -> Result<(HashMap<String, String>, String)> {
    let (mut master_key, new_master_key) = check_key();
    if new_master_key == false {
        let entered_master_key = prompt_for_master_key("Please enter master password: ");
        let entered_master_key_hash = hash_key(&entered_master_key);

        if master_key != entered_master_key_hash {
            println!("Incorrect master password");
            return Ok((HashMap::new(), master_key));
        }

        master_key = entered_master_key
    }

    if !Path::new(PASSWORD_DB_PATH).exists() {
        return Ok((HashMap::new(), master_key));
    }

    let file_data = std::fs::read(PASSWORD_DB_PATH).unwrap();
    let decrypted_data = decrypt_data(&file_data, &master_key)?;
    let password_db: HashMap<String, String> = serde_json::from_str(&decrypted_data).unwrap();

    return Ok((password_db, master_key));
}
