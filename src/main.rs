use clap::Parser;
use hiredavidparker::{about, greeting};

#[derive(Parser, Debug, PartialEq)]
#[command(name = "hiredavidparker")]
#[command(about = "David Parker's interactive resume for Warp")]
struct Cli {
    #[command(subcommand)]
    command: Option<Commands>,
}

#[derive(clap::Subcommand, Debug, PartialEq)]
enum Commands {
    /// Run the application with a welcome message
    Run,

    /// Display information about me
    About,
}

/// Process CLI arguments and return the resulting output message
pub fn process_args(args: &[String]) -> String {
    let cli = Cli::parse_from(args);

    match cli.command {
        Some(Commands::Run) => greeting(),
        Some(Commands::About) => about(),
        None => greeting(),
    }
}

fn main() {
    let args: Vec<String> = std::env::args().collect();
    println!("{}", process_args(&args));
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_cli_with_run_command() {
        let args = vec!["hiredavidparker".to_string(), "run".to_string()];
        assert_eq!(process_args(&args), "Hello Warp, I am David Parker.");
    }

    #[test]
    fn test_cli_with_no_command() {
        let args = vec!["hiredavidparker".to_string()];
        assert_eq!(process_args(&args), "Hello Warp, I am David Parker.");
    }

    #[test]
    fn test_cli_parser() {
        // Test the parser with "run" command
        let cli = Cli::parse_from(vec!["app", "run"]);
        assert_eq!(cli.command, Some(Commands::Run));

        // Test the parser with no command
        let cli = Cli::parse_from(vec!["app"]);
        assert_eq!(cli.command, None);
    }
}
