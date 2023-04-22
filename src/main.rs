use std::{env, path::PathBuf};

use clap::{Parser, Subcommand};
use efs_lib::project::Project;

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
        to: Option<Vec<PathBuf>>,
    },
    /// Check for errors
    Check,
    /// Verify project structure
    VerifyProject,
    /// Builds to targets when the project when files change
    Watch {
        #[arg(long)]
        to: Option<Vec<PathBuf>>,
    },
}

fn main() {
    let cli = Cli::parse();

    match cli.command {
        Commands::New { name } => {
            println!("new: {}", name);

            Project::new(name.clone(), name.into()).unwrap();
        }
        Commands::Init => {
            println!("init");

            let dir = env::current_dir().unwrap();

            Project::new(dir.file_name().unwrap().to_string_lossy().to_string(), dir).unwrap();
        }
        Commands::Build { to } => {
            println!("build: {:?}", to);
        }
        Commands::Check => {
            println!("check");
        }
        Commands::VerifyProject => {
            println!("verify project");
        }
        Commands::Watch { to } => {
            println!("watch: {:?}", to);
        }
    }
}
