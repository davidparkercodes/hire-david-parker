use std::fs;
use std::path::Path;
use std::error::Error;

pub mod tui;

/// Returns a greeting message
pub fn greeting() -> String {
    String::from("Hello Warp, I am David Parker.")
}

/// Load content from markdown files
pub fn load_content(filename: &str) -> String {
    let path = Path::new(env!("CARGO_MANIFEST_DIR"))
        .join("src")
        .join("static")
        .join("content")
        .join(filename);

    match fs::read_to_string(&path) {
        Ok(content) => content,
        Err(_) => format!("Error: Failed to load content from '{}'", filename),
    }
}

/// Returns the about content
pub fn about() -> String {
    load_content("about.md")
}

/// Returns the skills content
pub fn skills() -> String {
    load_content("skills.md")
}

/// Returns the projects content
pub fn projects() -> String {
    load_content("projects.md")
}

/// Returns the why Warp content
pub fn why_warp() -> String {
    load_content("why_warp.md")
}

/// Returns the welcome content
pub fn welcome() -> String {
    load_content("welcome.md")
}

/// Returns the timeline content
pub fn timeline() -> String {
    load_content("timeline.md")
}

/// Loads timeline data from JSON file
pub fn load_timeline_data() -> Result<Vec<TimelineEvent>, Box<dyn Error>> {
    let path = Path::new(env!("CARGO_MANIFEST_DIR"))
        .join("src")
        .join("static")
        .join("content")
        .join("timeline.json");
    
    let content = fs::read_to_string(&path)?;
    let timeline_events: Vec<TimelineEvent> = serde_json::from_str(&content)?;
    Ok(timeline_events)
}

/// Represents a timeline event
#[derive(Debug, Clone, serde::Deserialize, serde::Serialize)]
pub struct TimelineEvent {
    pub year: u32,
    pub title: String,
    pub organization: String,
    pub description: String,
    pub highlights: Vec<String>,
    pub technologies: Vec<String>,
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
