use rand::Rng;

const PASSWORD_CHARS: &str =
    "abcdefghijklmnopqrstuvwxyzABCDEFGHIJKLMNOPQRSTUVWXYZ0123456789!@#$%^&*?";

pub fn generate_password(length: u8) {
    let mut rng = rand::rngs::OsRng::default();

    let password: String = (0..length)
        .map(|_| {
            let idx = rng.gen::<usize>() % PASSWORD_CHARS.len();
            PASSWORD_CHARS.chars().nth(idx).unwrap()
        })
        .collect();

    println!("{}", password);
}
