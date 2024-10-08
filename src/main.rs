use std::path::Path;
use clap::{Parser, Subcommand};
use cmd::{cat_files, copy_file, create_file, delete_file};
use anyhow::{anyhow, Result};

pub mod cmd;

#[derive(Parser)]
#[command(about="Perform common file operations easily.")]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    #[command(about="Create a new file with optional text")]
    Create {
        #[arg(short)]
        text: Option<String>,
        #[arg(required(true))]
        filename: String,
    },
    #[command(about="Copy an existing file to a new location")]
    Copy {
        #[arg(required(true))]
        src_file: String,
        #[arg(required(true))]
        dst_file: String,
    },
    #[command(about="Concatenate two existing files into a new location")]
    Cat {
        #[arg(required(true))]
        src_file1: String,
        #[arg(required(true))]
        src_file2: String,
        #[arg(required(true))]
        dst_file: String,
    },
    #[command(about="Delete an existing file")]
    Del {
        #[arg(required(true))]
        filename: String,
    },
}
fn main() -> Result<()> {
    let cli = Cli::parse();

    let res = match &cli.command {
        Commands::Create { filename, text } => {
            create_file(Path::new(filename), text.as_deref())
        }
        Commands::Copy { src_file, dst_file } => {
            copy_file(Path::new(src_file), Path::new(dst_file))
        }
        Commands::Cat {
            dst_file,
            src_file1,
            src_file2,
        } => cat_files(
            Path::new(src_file1),
            Path::new(src_file2),
            Path::new(dst_file),
        ),
        Commands::Del { filename } => delete_file(Path::new(filename)),
    };

    match res {
        Ok(msg) => {
            println!("{}", msg);
            Ok(())
        }
        Err(e) => {
            Err(anyhow!(e.to_string()))
        }
    }
  
}
