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
    /// Gets a stored password
    Get { name: String },
    /// Removes a stored password
    Remove { name: String },
    /// Resets the master password
    Reset,
}

fn main() {
    let cli_args = CliArgs::parse();

    match cli_args.command {
        Commands::Generate { length } => commands::generate::generate_password(length),
        Commands::Store { password, name } => {
            match commands::store::store_password(&name, &password) {
                Ok(_) => {}
                Err(e) => eprintln!("Error storing password: {}", e),
            }
        }
        Commands::List => match commands::list::list_passwords() {
            Ok(_) => {}
            Err(e) => eprintln!("Error listing passwords: {}", e),
        },
        Commands::Get { name } => match commands::get::get_password(&name) {
            Ok(_) => {}
            Err(e) => eprintln!("Error getting password: {}", e),
        },
        Commands::Remove { name } => match commands::remove::remove_password(&name) {
            Ok(_) => {}
            Err(e) => eprintln!("Error removing password: {}", e),
        },
        Commands::Reset => match commands::reset::reset_master_password() {
            Ok(_) => {}
            Err(e) => eprintln!("Error resetting master password: {}", e),
        },
    }
}
