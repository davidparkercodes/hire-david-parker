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
            #[cfg(not(test))]
            run_tui()?;
            Ok(String::new())
        }
        Some(Commands::About) => Ok(about()),
        None => {
            #[cfg(not(test))]
            run_tui()?;
            Ok(String::new())
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
        let cli = Cli::parse_from(vec!["app", "run"]);
        assert_eq!(cli.command, Some(Commands::Run));

        let cli = Cli::parse_from(vec!["app", "about"]);
        assert_eq!(cli.command, Some(Commands::About));

        let cli = Cli::parse_from(vec!["app"]);
        assert_eq!(cli.command, None);
    }

    #[test]
    fn test_process_args_about() -> Result<(), Box<dyn Error>> {
        let args = vec![String::from("app"), String::from("about")];
        let result = process_args(&args)?;

        assert!(!result.is_empty());
        assert!(result.contains("About David Parker") || result.contains("Warp team"));

        Ok(())
    }

    #[test]
    fn test_process_args_run() -> Result<(), Box<dyn Error>> {
        let args = vec![String::from("app"), String::from("run")];
        let result = process_args(&args)?;

        assert!(result.is_empty());

        Ok(())
    }

    #[test]
    fn test_process_args_no_command() -> Result<(), Box<dyn Error>> {
        let args = vec![String::from("app")];
        let result = process_args(&args)?;

        assert!(result.is_empty());

        Ok(())
    }

    #[test]
    fn test_debug_for_commands() {
        let run_cmd = Commands::Run;
        let about_cmd = Commands::About;

        assert_eq!(format!("{:?}", run_cmd), "Run");
        assert_eq!(format!("{:?}", about_cmd), "About");
    }

    #[test]
    fn test_commands_eq() {
        assert_eq!(Commands::Run, Commands::Run);
        assert_eq!(Commands::About, Commands::About);
        assert_ne!(Commands::Run, Commands::About);
    }

    #[test]
    fn test_cli_debug() {
        let cli = Cli {
            command: Some(Commands::Run),
        };
        let debug_str = format!("{:?}", cli);

        assert!(debug_str.contains("Run"));
        assert!(debug_str.contains("command"));
    }

    #[test]
    fn test_cli_eq() {
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
