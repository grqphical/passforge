const PASSWORD_CHARS: &str =
    "abcdefghijklmnopqrstuvwxyzABCDEFGHIJKLMNOPQRSTUVWXYZ0123456789!@#$%^&*?";

pub fn generate_password(length: u8) {
    let password: String = (0..length)
        .map(|_| {
            let idx = rand::random::<usize>() % PASSWORD_CHARS.len();
            PASSWORD_CHARS.chars().nth(idx).unwrap()
        })
        .collect();

    println!("{}", password);
}
