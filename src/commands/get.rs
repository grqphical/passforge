use crate::decryption::read_password_database;

pub fn get_password(name: &String) {
    let (password_db, _) = read_password_database().unwrap();
    match password_db.get(name) {
        Some(password) => println!("{}: {}", name, password),
        None => println!("Password not found"),
    }
}
