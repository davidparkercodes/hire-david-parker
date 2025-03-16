mod common;

#[test]
fn test_cli_run_command() {
    let stdout = common::run_cli_with_args(&["run"]);
    assert!(stdout.contains("Hello Warp, I am David Parker."));
}

#[test]
fn test_cli_no_command() {
    // For testing with no command, we need to use std::process::Command directly
    let output = std::process::Command::new("cargo")
        .args(["run"])
        .output()
        .expect("Failed to execute command");

    let stdout = String::from_utf8(output.stdout).expect("Failed to parse stdout");
    assert!(stdout.contains("Hello Warp, I am David Parker."));
}