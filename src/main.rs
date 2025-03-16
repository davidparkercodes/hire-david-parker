use clap::Parser;
use hiredavidparker::{greeting, about};

#[derive(Parser)]
#[command(name = "hiredavidparker")]
#[command(about = "David Parker's interactive resume for Warp")]
struct Cli {
    #[command(subcommand)]
    command: Option<Commands>,
}

#[derive(clap::Subcommand)]
enum Commands {
    /// Run the application with a welcome message
    Run,
    
    /// Display information about me
    About,
}

fn main() {
    let cli = Cli::parse();
    
    match cli.command {
        Some(Commands::Run) => {
            println!("{}", greeting());
        }
        Some(Commands::About) => {
            println!("{}", about());
        }
        None => {
            println!("{}", greeting());
        }
    }
}
