use clap::Parser;
use hire_david_parker::{about, run_tui};
use std::error::Error;

#[derive(Parser, Debug, PartialEq)]
#[command(name = "hire-david-parker")]
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
            // In test mode, we don't actually want to run the TUI
            #[cfg(not(test))]
            run_tui()?;
            Ok(String::new()) // TUI handles its own output
        }
        Some(Commands::About) => Ok(about()),
        None => {
            // In test mode, we don't actually want to run the TUI
            #[cfg(not(test))]
            run_tui()?;
            Ok(String::new()) // TUI handles its own output
        }
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
    use std::error::Error;

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

    #[test]
    fn test_process_args_about() -> Result<(), Box<dyn Error>> {
        // When using the "about" command, process_args should return the about content
        let args = vec![String::from("app"), String::from("about")];
        let result = process_args(&args)?;

        // The about content should contain specific text
        assert!(!result.is_empty());
        assert!(result.contains("About David Parker") || result.contains("Warp team"));

        Ok(())
    }

    #[test]
    fn test_process_args_run() -> Result<(), Box<dyn Error>> {
        // When using the "run" command, process_args should return empty string
        let args = vec![String::from("app"), String::from("run")];
        let result = process_args(&args)?;

        // Result should be empty (TUI handles output)
        assert!(result.is_empty());

        Ok(())
    }

    #[test]
    fn test_process_args_no_command() -> Result<(), Box<dyn Error>> {
        // When no command is provided, it defaults to running the TUI
        let args = vec![String::from("app")];
        let result = process_args(&args)?;

        // Result should be empty (TUI handles output)
        assert!(result.is_empty());

        Ok(())
    }

    #[test]
    fn test_debug_for_commands() {
        // Test that Commands enum implements Debug
        let run_cmd = Commands::Run;
        let about_cmd = Commands::About;

        assert_eq!(format!("{:?}", run_cmd), "Run");
        assert_eq!(format!("{:?}", about_cmd), "About");
    }

    #[test]
    fn test_commands_eq() {
        // Test PartialEq implementation for Commands
        assert_eq!(Commands::Run, Commands::Run);
        assert_eq!(Commands::About, Commands::About);
        assert_ne!(Commands::Run, Commands::About);
    }

    #[test]
    fn test_cli_debug() {
        // Test Debug implementation for Cli
        let cli = Cli {
            command: Some(Commands::Run),
        };
        let debug_str = format!("{:?}", cli);

        assert!(debug_str.contains("Run"));
        assert!(debug_str.contains("command"));
    }

    #[test]
    fn test_cli_eq() {
        // Test PartialEq implementation for Cli
        let cli1 = Cli {
            command: Some(Commands::Run),
        };
        let cli2 = Cli {
            command: Some(Commands::Run),
        };
        let cli3 = Cli {
            command: Some(Commands::About),
        };

        assert_eq!(cli1, cli2);
        assert_ne!(cli1, cli3);
    }
}
