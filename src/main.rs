use std::path::PathBuf;

use clap::{Parser, Subcommand};

/*
Usage:
    efsc new <name>
    efsc init
    efsc build
    efsc build --to <path>
    efsc check
*/

#[derive(Parser)]
#[command(author, version, about, long_about = None)]
#[command(propagate_version = true)]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    /// Creates new efs project
    New { name: String },
    /// Creates new efs project in the current directory
    Init,
    /// Builds the project to the targets
    Build {
        #[arg(long)]
        to: Option<PathBuf>
    },
    /// Check for errors
    Check,
}

fn main() {
    let cli = Cli::parse();

    match cli.command {
        Commands::New { name } => {
            println!("new: {}", name);
        }
        Commands::Init => {
            println!("init");
        }
        Commands::Build { to } => {
            println!("build: {:?}", to);
        }
        Commands::Check => {
            println!("check");
        }
    }
}
