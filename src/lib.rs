use std::fs;
use std::path::Path;
use std::error::Error;

pub mod tui;

/// Returns a greeting message
pub fn greeting() -> String {
    String::from("Hello Warp, I am David Parker.")
}

/// Returns the about content from a static file
pub fn about() -> String {
    let about_path = Path::new(env!("CARGO_MANIFEST_DIR"))
        .join("src")
        .join("static")
        .join("about.txt");

    match fs::read_to_string(&about_path) {
        Ok(content) => content,
        Err(_) => String::from("Error: About information could not be loaded."),
    }
}

/// Runs the interactive TUI application
pub fn run_tui() -> Result<(), Box<dyn Error>> {
    tui::app::run()?;
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_greeting() {
        assert_eq!(greeting(), "Hello Warp, I am David Parker.");
    }

    #[test]
    fn test_about_content() {
        let about_content = about();
        assert!(about_content.contains("About David Parker"));
        assert!(about_content.contains("Warp team"));
    }
}
