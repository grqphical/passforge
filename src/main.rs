mod commands;
mod hashing;

use clap::{Parser, Subcommand};

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
}

fn main() {
    let cli_args = CliArgs::parse();

    match cli_args.command {
        Commands::Generate { length } => commands::generate::generate_password(length),
        Commands::Store { password, name } => {
            commands::store::store_password(&name, &password).unwrap()
        }
    }
}
