// Common test utilities can be added here

/// Run the CLI with the given arguments and return the stdout output
pub fn run_cli_with_args(args: &[&str]) -> String {
    let output = std::process::Command::new("cargo")
        .arg("run")
        .arg("--")
        .args(args)
        .output()
        .expect("Failed to execute command");
    
    String::from_utf8(output.stdout).expect("Failed to parse stdout")
}