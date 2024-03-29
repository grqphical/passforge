# PassForge - A CLI Password Manager written in Rust

PassForge uses AES encryption to encrypt your stored passwords and a SHA256 hashing algorithim to store the master password.

## Functional Goals

1. Store passwords in an encrypted file

   - Passwords are stored in a file that is encrypted using AES encryption
   - Passwords are decrypted using the master password

2. Generate passwords
   - Passwords can be generated using a variety of options
   - Passwords are generated using the `OsRng` random number generator from the `rand` crate
