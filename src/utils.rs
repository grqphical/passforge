use anyhow::{format_err, Result};
use rpassword::read_password;
use std::{io::Write, path::Path};

use crate::hashing::hash_key;

const KEY_FILE_PATH: &str = "MASTERKEY";

pub fn hex_to_bytes(hex: &str) -> Result<Vec<u8>> {
    hex::decode(hex).map_err(|e| format_err!("Error decoding hex: {}", e))
}

pub fn prompt_for_master_key() -> String {
    print!("Type master password: ");
    std::io::stdout().flush().unwrap();
    let password = read_password().unwrap();
    return password;
}

fn prompt_for_master_key_creation() -> String {
    print!("Type master password: ");
    std::io::stdout().flush().unwrap();
    let password = read_password().unwrap();

    loop {
        print!("Confirm Password: ");
        std::io::stdout().flush().unwrap();
        let confirmed_password = read_password().unwrap();

        if confirmed_password == password {
            break;
        } else {
            println!("Passwords do not match. Try again.");
        }
    }

    return password;
}

/// Checks if a master password has been set. If not it prompts the user to set one
///
/// Returns the hash of the master password and a boolean stating if the password was just created
pub fn check_key() -> (String, bool) {
    if !Path::new(KEY_FILE_PATH).exists() {
        let password = prompt_for_master_key_creation();
        let hash = hash_key(&password);
        std::fs::write(KEY_FILE_PATH, format!("// DO NOT MODIFY THIS FILE OR ELSE ALL OF YOUR PASSWORDS WILL BE LOST. RESET YOUR MASTER KEY WITH 'reset'\n{}", hash)).unwrap();
        return (hash, true);
    } else {
        let file_data = std::fs::read_to_string(KEY_FILE_PATH).unwrap();
        return (
            file_data.split("\n").collect::<Vec<&str>>()[1].to_string(),
            false,
        );
    }
}
