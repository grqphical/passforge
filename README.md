# PassForge - A CLI Password Manager written in Rust

PassForge uses AES encryption to encrypt your stored passwords and a SHA256 hashing algorithim to store the master password

**NOTE:** This is a personal project and should not be used to store sensitive information on sensitive systems. Any security vulnerabilities should be reported to the author. I am not a security expert and this project is not intended to be a secure password manager. This project is intended to be a learning experience for me to learn more about Rust and encryption.

## Functional Goals

1. Store passwords in an encrypted file

   - Passwords are stored in a file that is encrypted using AES encryption in the JSON format
   - Passwords are decrypted using the master password

2. Generate passwords

   - Passwords can be generated with a custom length
   - Passwords are generated using the `OsRng` random number generator from the `rand` crate

3. Access/Modify passwords in encrypted file
   - Passwords can be accessed and modified using the CLI

## Usage

```bash
passforge [SUBCOMMAND]
```

### Subcommands

- `store [NAME] [PASSWORD]` - Add a password to the password store
- `get [NAME]` - Get a password from the password store
- `list` - List all passwords in the password store
- `remove [NAME]` - Remove a password from the password store
- `generate` - Generate a new password. Specify length of password with `-l` flag
- `reset` - Reset the password store making all passwords currently stored unrecoverable
