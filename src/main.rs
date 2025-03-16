use clap::Parser;

#[derive(Parser)]
#[command(name = "hiredavidparker")]
#[command(about = "David Parker's interactive resume for Warp")]
struct Cli {
    #[command(subcommand)]
    command: Option<Commands>,
}

#[derive(clap::Subcommand)]
enum Commands {
    /// Run the application
    Run,
}

fn main() {
    let cli = Cli::parse();
    
    match cli.command {
        Some(Commands::Run) => {
            println!("Hello Warp, I am David Parker.");
        }
        None => {
            println!("Hello Warp, I am David Parker.");
        }
    }
}
