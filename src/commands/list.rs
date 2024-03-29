use crate::decryption::read_password_database;
use anyhow::Result;

pub fn list_passwords() -> Result<()> {
    let (password_db, _) = read_password_database()?;
    for (name, value) in password_db {
        println!("{}: {}", name, value);
    }

    return Ok(());
}
