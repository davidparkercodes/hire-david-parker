mod common;

#[test]
fn test_cli_about_command() {
    let stdout = common::run_cli_with_args(&["about"]);
    assert!(stdout.contains("About David Parker"));
    assert!(stdout.contains("Warp team"));
}

// Note: We can't easily test the TUI in an automated way since it requires interaction,
// so we've removed the tests for the "run" command that now launches the TUI
