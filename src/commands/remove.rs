use crate::{decryption::read_password_database, encryption::write_password_database};

pub fn remove_password(name: &String) {
    let (mut password_db, master_hash) = read_password_database().unwrap();
    match password_db.remove(name) {
        Some(_) => {
            write_password_database(password_db, master_hash).unwrap();
            println!("Password removed");
        }
        None => println!("Password not found"),
    }
}
