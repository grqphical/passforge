use crate::utils::{check_key, prompt_for_master_key};
use crate::{decryption::read_password_database, hashing::hash_key};

pub fn list_passwords() {
    let (mut master_hash, new_master_key) = check_key();
    if new_master_key == false {
        let entered_master_key_hash = hash_key(&prompt_for_master_key());

        if master_hash != entered_master_key_hash {
            println!("Incorrect master password");
            return;
        }

        master_hash = entered_master_key_hash;
    }
    let password_db = read_password_database(&master_hash).unwrap();
    for (name, value) in password_db {
        println!("{}: {}", name, value);
    }
}
