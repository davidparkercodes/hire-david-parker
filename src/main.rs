use clap::Parser;
use hiredavidparker::{about, run_tui};
use std::error::Error;

#[derive(Parser, Debug, PartialEq)]
#[command(name = "hiredavidparker")]
#[command(about = "David Parker's interactive resume for Warp")]
struct Cli {
    #[command(subcommand)]
    command: Option<Commands>,
}

#[derive(clap::Subcommand, Debug, PartialEq)]
enum Commands {
    /// Run the interactive TUI application
    Run,

    /// Display information about me
    About,
}

/// Process CLI arguments and return the resulting output message
pub fn process_args(args: &[String]) -> Result<String, Box<dyn Error>> {
    let cli = Cli::parse_from(args);

    match cli.command {
        Some(Commands::Run) => {
            run_tui()?;
            Ok(String::new())  // TUI handles its own output
        },
        Some(Commands::About) => Ok(about()),
        None => {
            run_tui()?;
            Ok(String::new())  // TUI handles its own output
        },
    }
}

fn main() -> Result<(), Box<dyn Error>> {
    let args: Vec<String> = std::env::args().collect();
    let result = process_args(&args)?;
    if !result.is_empty() {
        println!("{}", result);
    }
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_cli_parser() {
        // Test the parser with "run" command
        let cli = Cli::parse_from(vec!["app", "run"]);
        assert_eq!(cli.command, Some(Commands::Run));

        // Test the parser with "about" command
        let cli = Cli::parse_from(vec!["app", "about"]);
        assert_eq!(cli.command, Some(Commands::About));

        // Test the parser with no command
        let cli = Cli::parse_from(vec!["app"]);
        assert_eq!(cli.command, None);
    }
}
