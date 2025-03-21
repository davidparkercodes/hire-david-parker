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


/// Returns the welcome content
pub fn welcome() -> String {
    load_content("welcome.md")
}

/// Returns the timeline content
pub fn timeline() -> String {
    load_content("timeline.md")
}

/// Returns the contact content
pub fn contact() -> String {
    load_content("contact.md")
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

/// Project link structure
#[derive(Debug, Clone, serde::Deserialize, serde::Serialize)]
pub struct ProjectLink {
    pub text: String,
    pub url: String,
}

/// Project links data structure
#[derive(Debug, Clone, serde::Deserialize, serde::Serialize)]
pub struct ProjectLinks {
    pub links: Vec<ProjectLink>,
}

/// Loads project links data from JSON file
pub fn load_project_links() -> Result<ProjectLinks, Box<dyn Error>> {
    let path = Path::new(env!("CARGO_MANIFEST_DIR"))
        .join("src")
        .join("static")
        .join("content")
        .join("projects.json");
    
    let content = fs::read_to_string(&path)?;
    let project_links: ProjectLinks = serde_json::from_str(&content)?;
    Ok(project_links)
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
#[cfg(not(test))]
pub fn run_tui() -> Result<(), Box<dyn Error>> {
    tui::run()?;
    Ok(())
}

/// Test version of run_tui that doesn't actually launch the TUI
#[cfg(test)]
pub fn run_tui() -> Result<(), Box<dyn Error>> {
    // In test mode, just return OK without running the TUI
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

    #[test]
    fn test_skills_content() {
        let skills_content = skills();
        assert!(!skills_content.is_empty());
        assert!(!skills_content.contains("Error: Failed to load content from 'skills.md'"));
    }

    #[test]
    fn test_projects_content() {
        let projects_content = projects();
        assert!(!projects_content.is_empty());
        assert!(!projects_content.contains("Error: Failed to load content from 'projects.md'"));
    }


    #[test]
    fn test_welcome_content() {
        let welcome_content = welcome();
        assert!(!welcome_content.is_empty());
        assert!(!welcome_content.contains("Error: Failed to load content from 'welcome.md'"));
    }

    #[test]
    fn test_timeline_content() {
        let timeline_content = timeline();
        assert!(!timeline_content.is_empty());
        assert!(!timeline_content.contains("Error: Failed to load content from 'timeline.md'"));
    }
    
    #[test]
    fn test_contact_content() {
        let contact_content = contact();
        assert!(!contact_content.is_empty());
        assert!(!contact_content.contains("Error: Failed to load content from 'contact.md'"));
    }

    #[test]
    fn test_load_timeline_data() {
        let result = load_timeline_data();
        assert!(result.is_ok());
        
        let timeline_events = result.unwrap();
        assert!(!timeline_events.is_empty());
        
        let first_event = &timeline_events[0];
        assert!(first_event.year > 0);
        assert!(!first_event.title.is_empty());
        assert!(!first_event.organization.is_empty());
        assert!(!first_event.description.is_empty());
    }

    #[test]
    fn test_load_content_error_handling() {
        let content = load_content("nonexistent_file.md");
        assert!(content.contains("Error: Failed to load content from 'nonexistent_file.md'"));
    }
    
    #[test]
    fn test_run_tui() {
        // In test mode, run_tui() should just return Ok without actually running the TUI
        let result = run_tui();
        assert!(result.is_ok());
    }
}
