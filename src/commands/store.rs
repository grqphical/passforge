use crate::decryption::read_password_database;
use crate::encryption::write_password_database;
use anyhow::Result;

pub fn store_password(name: &String, password: &String) -> Result<()> {
    let (mut password_db, master_key) = read_password_database()?;
    password_db.insert(name.to_string(), password.to_string());

    write_password_database(password_db, master_key)?;

    return Ok(());
}
