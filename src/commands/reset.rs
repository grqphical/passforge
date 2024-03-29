use crate::{
    hashing::hash_key,
    utils::{check_key, prompt_for_master_key},
};
use anyhow::Result;
use std::io::Write;

pub fn reset_master_password() -> Result<()> {
    println!("WARNING: This will invalidate the encryption on all stored passwords rendering them unusable. This action cannot be undone.");
    print!("Are you sure you want to reset the master password? (y/n): ");
    let mut input = String::new();
    std::io::stdout().flush()?;
    std::io::stdin().read_line(&mut input)?;
    if input.trim() != "y" {
        return Ok(());
    }

    let entered_master_key_hash =
        hash_key(&prompt_for_master_key("Enter current master password: "));
    let (current_hash, is_new) = check_key();
    if is_new {
        return Ok(());
    }

    if entered_master_key_hash != current_hash {
        println!("Master password is incorrect.");
        return Ok(());
    }

    std::fs::remove_file("MASTERKEY")?;
    check_key();
    println!("Master password reset. All passwords have been lost.");

    return Ok(());
}
