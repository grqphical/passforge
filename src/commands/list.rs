use crate::decryption::read_password_database;

pub fn list_passwords() {
    let (password_db, _) = read_password_database().unwrap();
    for (name, value) in password_db {
        println!("{}: {}", name, value);
    }
}
