use crate::{decryption::read_password_database, encryption::write_password_database};
use anyhow::Result;

pub fn remove_password(name: &String) -> Result<()> {
    let (mut password_db, master_key) = read_password_database()?;
    match password_db.remove(name) {
        Some(_) => {
            write_password_database(password_db, master_key)?;
            println!("Password removed");
        }
        None => println!("Password not found"),
    }

    return Ok(());
}
