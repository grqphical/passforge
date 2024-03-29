mod commands;
mod decryption;
mod encryption;
mod hashing;

const PASSWORD_DB_PATH: &str = "passwords.db";

use std::io::Write;

use clap::{Parser, Subcommand};
use rpassword::read_password;

#[derive(Parser, Debug)]
struct CliArgs {
    #[command(subcommand)]
    pub command: Commands,
}

#[derive(Subcommand, Debug)]
enum Commands {
    /// Generates a password
    Generate {
        /// Length of password
        #[arg(short, long, default_value_t = 12)]
        length: u8,
    },
    Store {
        /// Identifier for password
        name: String,

        /// Password to store
        password: String,
    },
    List,
}

fn prompt_for_master_key() -> String {
    print!("Type master password: ");
    std::io::stdout().flush().unwrap();
    let password = read_password().unwrap();
    return password;
}

fn main() {
    let cli_args = CliArgs::parse();

    match cli_args.command {
        Commands::Generate { length } => commands::generate::generate_password(length),
        Commands::Store { password, name } => {
            commands::store::store_password(&name, &password).unwrap()
        }
        Commands::List => commands::list::list_passwords(),
    }
}
