use clap::{Parser, Subcommand};
use espanol::builder;

#[derive(Parser, Debug)]
#[command(name = "Espanol")]
#[command(version = "0.0.1")]
#[command(about = "Español Dictionary for CLI", long_about = None)]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand, Debug)]
enum Commands {
    /// Build the dictionary database
    Build {
        /// Optional path to the dictionary file
        #[arg(short, long)]
        path: Option<String>,
    },
    /// Lookup a word in the dictionary
    Lookup {
        /// The word to search for
        word: String,
        /// Optional path to the dictionary file
        #[arg(short, long)]
        path: Option<String>,
    },
}

fn main() {
    let args = Cli::parse();

    match args.command {
        Commands::Build { path } => builder::run(path),
        Commands::Lookup { word, path } => {
            let dict_path = path.unwrap_or_else(|| "~/.espanol/dictionary.db".to_string());
            println!(
                "Looking up word: '{}' in dictionary at: {}",
                word, dict_path
            );
            // Implement the lookup word logic here
        }
    }
}
