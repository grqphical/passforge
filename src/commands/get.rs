use crate::decryption::read_password_database;
use anyhow::Result;

pub fn get_password(name: &String) -> Result<()> {
    let (password_db, _) = read_password_database()?;
    match password_db.get(name) {
        Some(password) => println!("{}: {}", name, password),
        None => println!("Password not found"),
    }

    return Ok(());
}
