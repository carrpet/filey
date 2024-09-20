use clap::{Parser, Subcommand};

#[derive(Parser)]
#[command(about)]
struct Cli {
   
   #[command(subcommand)]
    command: Option<Commands>
}

#[derive(Subcommand)]
enum Commands {
    Create {
        #[arg(short)]
        text: Option<String>,
        #[arg()]
        filename: String
    },
    Copy {
        #[arg()]
        src_file: String,
        #[arg()]
        dst_file: String
    },
    Cat {
        #[arg()]
        src_file1: String,
        #[arg()]
        src_file2: String,
        #[arg()]
        dst_file: String
    },

    Del {
        #[arg()]
        filename: String
    }


}
fn main() {
    let cli = Cli::parse();

    match &cli.command {
        Some(Commands::Create { filename, text }) => {
            println!("{} {:?}", filename, text)
        },
        Some(Commands::Copy {src_file, dst_file}) => {
            println!("source:{}, dst: {}", src_file, dst_file)
        },
        Some(Commands::Cat { dst_file, src_file1, src_file2 }) => {
            println!("cat from {} and {} to: {}", src_file1, src_file2, dst_file)
        },
        Some(Commands::Del { filename: target }) => {
            println!("delete: {}", target)
        }
        None => println!("No subcommand!"),
    }
}
