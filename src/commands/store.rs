use crate::decryption::read_password_database;
use crate::encryption::write_password_database;
use crate::hashing::hash_key;
use crate::utils::{check_key, prompt_for_master_key};
use anyhow::Result;

pub fn store_password(name: &String, password: &String) -> Result<()> {
    let (mut master_hash, new_master_key) = check_key();
    if new_master_key == false {
        let entered_master_key_hash = hash_key(&prompt_for_master_key());

        if master_hash != entered_master_key_hash {
            println!("Incorrect master password");
            return Ok(());
        }

        master_hash = entered_master_key_hash;
    }

    let mut password_db = read_password_database(&master_hash)?;
    password_db.insert(name.to_string(), password.to_string());

    write_password_database(password_db, master_hash)?;

    return Ok(());
}
