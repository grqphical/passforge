mod commands;
mod decryption;
mod encryption;
mod hashing;
mod utils;
use clap::{Parser, Subcommand};

const PASSWORD_DB_PATH: &str = "passwords.db";

#[derive(Parser, Debug)]
struct CliArgs {
    #[command(subcommand)]
    pub command: Commands,
}

#[derive(Subcommand, Debug)]
#[command(version, about = "Simple CLI Password Manager", long_about = None)]
enum Commands {
    /// Generates a password
    Generate {
        /// Length of password
        #[arg(short, long, default_value_t = 12)]
        length: u8,
    },
    /// Stores a password
    Store {
        /// Identifier for password
        name: String,

        /// Password to store
        password: String,
    },
    /// Lists all stored passwords
    List,
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
