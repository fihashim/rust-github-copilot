// A command-line tool to play Marco Polo
use clap::{Parser, Subcommand};

#[derive(Parser)]
#[command(version = "1.0", author = "Noah Gift", about = "A Marco Polo game")]
struct Cli {
    #[command(subcommand)]
    command: Option<Commands>,
}

#[derive(Subcommand)]
enum Commands {
    #[command(about = "Play the Marco Polo game")]
    Play {
        #[arg(short, long)]
        name: String,
    },
}

fn main() {
    let args = Cli::parse();
    match args.command {
        Some(Commands::Play { name }) => {
            let result = hello_marco::marco_polo(&name);
            println!("{}", result);
        }
        None => println!("No subcommand was used"),
    }
}
