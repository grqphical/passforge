use crate::hashing::hash_key;
use rpassword::read_password;
use std::{io::Write, path::Path};

const KEY_FILE_PATH: &str = "MASTERKEY";

/// Prompts the user for a master key
pub fn prompt_for_master_key(prompt: &'static str) -> String {
    print!("{}", prompt);
    std::io::stdout().flush().unwrap();
    let password = read_password().unwrap();
    return password;
}

/// Converts a slice to a fixed size array
pub fn slice_to_fixed_array<T: Copy + Default, const N: usize>(slice: &[T]) -> [T; N] {
    let mut array: [T; N] = [T::default(); N];
    let len = slice.len().min(N);
    array[..len].clone_from_slice(&slice[..len]);
    array
}

/// Prompts the user to create a master key
fn prompt_for_master_key_creation() -> String {
    print!("Type master password: ");
    std::io::stdout().flush().unwrap();
    let password = read_password().unwrap();

    // Keep the password under 32 chars to avoid issues with the encryption library (AES-256)
    if password.len() < 8 || password.len() > 32 {
        println!("Password must be at least 8 characters long and less than 32 characters long.");
        return prompt_for_master_key_creation();
    }

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
/// Returns the hash of the master password if one already exists else it returns the password.
/// It also returns a boolean indicating that a new master password was created
pub fn check_key() -> (String, bool) {
    if !Path::new(KEY_FILE_PATH).exists() {
        let password = prompt_for_master_key_creation();
        let hash = hash_key(&password);
        std::fs::write(KEY_FILE_PATH, format!("// DO NOT MODIFY THIS FILE OR ELSE ALL OF YOUR PASSWORDS WILL BE LOST. RESET YOUR MASTER KEY WITH 'reset'\n{}", hash)).unwrap();
        return (password, true);
    } else {
        let file_data = std::fs::read_to_string(KEY_FILE_PATH).unwrap();
        return (
            file_data.split("\n").collect::<Vec<&str>>()[1].to_string(),
            false,
        );
    }
}
