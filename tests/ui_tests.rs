mod common;

use hiredavidparker::tui::state::App;
use hiredavidparker::tui::models::{DisplayMode, TimelineEvent, SkillCategory, Skill, Link};
use ratatui::{
    backend::TestBackend,
    buffer::Buffer,
    Terminal,
};
use hiredavidparker::tui::ui;

#[test]
fn test_ui_menu_sidebar_rendering() {
    // Setup test terminal
    let backend = TestBackend::new(80, 30);
    let mut terminal = Terminal::new(backend).unwrap();
    let mut app = App::new();
    
    // Set menu mode
    app.display_mode = DisplayMode::Menu;
    
    // Basic test to ensure UI rendering doesn't panic
    terminal.draw(|f| {
        ui::render(f, &mut app);
    }).unwrap();
    
    // Get the buffer to check contents
    let buffer = terminal.backend().buffer().clone();
    
    // Check that the menu items are rendered in the buffer
    let buffer_content = buffer_to_string(&buffer);
    assert!(buffer_content.contains("Menu"));
    assert!(buffer_content.contains("About Me"));
    assert!(buffer_content.contains("Skills"));
    assert!(buffer_content.contains("Projects"));
    assert!(buffer_content.contains("Timeline"));
}

#[test]
fn test_ui_about_rendering() {
    // Setup test terminal
    let backend = TestBackend::new(80, 30);
    let mut terminal = Terminal::new(backend).unwrap();
    let mut app = App::new();
    
    // Set about mode
    app.display_mode = DisplayMode::About;
    
    // Basic test to ensure UI rendering doesn't panic
    terminal.draw(|f| {
        ui::render(f, &mut app);
    }).unwrap();
    
    // Get the buffer to check contents
    let buffer = terminal.backend().buffer().clone();
    
    // Check that the about section is rendered
    let buffer_content = buffer_to_string(&buffer);
    assert!(buffer_content.contains("About Me"));
    assert!(buffer_content.contains("Menu"));
}

#[test]
fn test_ui_skills_rendering() {
    // Setup test terminal
    let backend = TestBackend::new(80, 30);
    let mut terminal = Terminal::new(backend).unwrap();
    let mut app = App::new();
    
    // Set skills mode
    app.display_mode = DisplayMode::Skills;
    
    // Basic test to ensure UI rendering doesn't panic
    terminal.draw(|f| {
        ui::render(f, &mut app);
    }).unwrap();
    
    // Get the buffer to check contents
    let buffer = terminal.backend().buffer().clone();
    
    // Check that the skills section is rendered
    let buffer_content = buffer_to_string(&buffer);
    assert!(buffer_content.contains("Skills"));
}

#[test]
fn test_ui_timeline_rendering() {
    // Setup test terminal
    let backend = TestBackend::new(80, 30);
    let mut terminal = Terminal::new(backend).unwrap();
    let mut app = App::new();
    
    // Set timeline mode
    app.display_mode = DisplayMode::Timeline;
    
    // Make sure we have timeline events
    if app.timeline_events.is_empty() {
        app.timeline_events.push(TimelineEvent {
            year: 2022,
            event_type: hiredavidparker::tui::models::TimelineType::Career,
            title: "Test Title".to_string(),
            organization: "Test Org".to_string(),
            description: "Test Description".to_string(),
            highlights: Some(vec!["Highlight 1".to_string()]),
            technologies: Some(vec!["Tech 1".to_string()]),
        });
    }
    
    // Basic test to ensure UI rendering doesn't panic
    terminal.draw(|f| {
        ui::render(f, &mut app);
    }).unwrap();
    
    // Get the buffer to check contents
    let buffer = terminal.backend().buffer().clone();
    
    // Check that the timeline section is rendered
    let buffer_content = buffer_to_string(&buffer);
    assert!(buffer_content.contains("Career Timeline"));
}

#[test]
fn test_ui_skills_visual_rendering() {
    // Setup test terminal
    let backend = TestBackend::new(80, 30);
    let mut terminal = Terminal::new(backend).unwrap();
    let mut app = App::new();
    
    // Set skills visual mode
    app.display_mode = DisplayMode::SkillsVisual;
    
    // Add skill categories if needed
    if app.skills_data.categories.is_empty() {
        app.skills_data.categories.push(SkillCategory {
            name: "Test Category".to_string(),
            skills: vec![
                Skill {
                    name: "Test Skill".to_string(),
                    level: 80,
                }
            ],
        });
    }
    
    // Basic test to ensure UI rendering doesn't panic
    terminal.draw(|f| {
        ui::render(f, &mut app);
    }).unwrap();
    
    // Get the buffer to check contents
    let buffer = terminal.backend().buffer().clone();
    
    // Check that the skills visual section is rendered
    let buffer_content = buffer_to_string(&buffer);
    assert!(buffer_content.contains("Skills:"));
}

#[test]
fn test_ui_welcome_rendering() {
    // Setup test terminal
    let backend = TestBackend::new(80, 30);
    let mut terminal = Terminal::new(backend).unwrap();
    let mut app = App::new();
    
    // Set menu mode to trigger welcome rendering
    app.display_mode = DisplayMode::Menu;
    app.menu_index = 0; // Not coming from timeline view
    app.previous_mode = DisplayMode::Menu;
    
    // Basic test to ensure UI rendering doesn't panic
    terminal.draw(|f| {
        ui::render(f, &mut app);
    }).unwrap();
    
    // Get the buffer to check contents
    let buffer = terminal.backend().buffer().clone();
    
    // Check that the welcome content is rendered
    let buffer_content = buffer_to_string(&buffer);
    assert!(buffer_content.contains("Instructions"));
}

#[test]
fn test_ui_projects_rendering() {
    // Setup test terminal
    let backend = TestBackend::new(80, 30);
    let mut terminal = Terminal::new(backend).unwrap();
    let mut app = App::new();
    
    // Set projects mode
    app.display_mode = DisplayMode::Projects;
    
    // Basic test to ensure UI rendering doesn't panic
    terminal.draw(|f| {
        ui::render(f, &mut app);
    }).unwrap();
    
    // Get the buffer to check contents
    let buffer = terminal.backend().buffer().clone();
    
    // Check that the projects content is rendered
    let buffer_content = buffer_to_string(&buffer);
    assert!(buffer_content.contains("Projects"));
}

#[test]
fn test_ui_project_links_rendering() {
    // Setup test terminal
    let backend = TestBackend::new(80, 30);
    let mut terminal = Terminal::new(backend).unwrap();
    let mut app = App::new();
    
    // Set project links mode
    app.display_mode = DisplayMode::ProjectLinks;
    
    // Inject test links into projects content
    app.projects_content = "# Test Project\n[Link 1](https://example.com)\n[Link 2](https://test.com)".to_string();
    
    // Basic test to ensure UI rendering doesn't panic
    terminal.draw(|f| {
        ui::render(f, &mut app);
    }).unwrap();
    
    // Get the buffer to check contents
    let buffer = terminal.backend().buffer().clone();
    
    // Check that the project links are rendered
    let buffer_content = buffer_to_string(&buffer);
    assert!(buffer_content.contains("Project Links"));
}

#[test]
fn test_ui_why_warp_rendering() {
    // Setup test terminal
    let backend = TestBackend::new(80, 30);
    let mut terminal = Terminal::new(backend).unwrap();
    let mut app = App::new();
    
    // Set why warp mode
    app.display_mode = DisplayMode::WhyWarp;
    
    // Basic test to ensure UI rendering doesn't panic
    terminal.draw(|f| {
        ui::render(f, &mut app);
    }).unwrap();
    
    // Get the buffer to check contents
    let buffer = terminal.backend().buffer().clone();
    
    // Check that the why warp content is rendered
    let buffer_content = buffer_to_string(&buffer);
    assert!(buffer_content.contains("Why Warp?"));
}

#[test]
fn test_ui_timeline_details_rendering() {
    // Setup test terminal
    let backend = TestBackend::new(80, 30);
    let mut terminal = Terminal::new(backend).unwrap();
    let mut app = App::new();
    
    // Set timeline mode
    app.display_mode = DisplayMode::Timeline;
    
    // Add timeline events if empty
    if app.timeline_events.is_empty() {
        app.timeline_events.push(TimelineEvent {
            year: 2021,
            event_type: hiredavidparker::tui::models::TimelineType::Career,
            title: "Test Title 1".to_string(),
            organization: "Test Org 1".to_string(),
            description: "Test Description 1".to_string(),
            highlights: Some(vec!["Highlight 1".to_string(), "Highlight 2".to_string()]),
            technologies: Some(vec!["Tech 1".to_string(), "Tech 2".to_string()]),
        });
        
        app.timeline_events.push(TimelineEvent {
            year: 2022,
            event_type: hiredavidparker::tui::models::TimelineType::Career,
            title: "Test Title 2".to_string(),
            organization: "Test Org 2".to_string(),
            description: "Test Description 2".to_string(),
            highlights: Some(vec!["Highlight A".to_string(), "Highlight B".to_string()]),
            technologies: Some(vec!["Tech A".to_string(), "Tech B".to_string()]),
        });
    }
    
    // Set the timeline index to the first event
    app.timeline_index = 0;
    
    // Basic test to ensure UI rendering doesn't panic
    terminal.draw(|f| {
        ui::render(f, &mut app);
    }).unwrap();
    
    // Get the buffer to check contents
    let buffer = terminal.backend().buffer().clone();
    
    // Check that the timeline details are rendered
    let buffer_content = buffer_to_string(&buffer);
    let event = &app.timeline_events[0];
    
    assert!(buffer_content.contains(&event.title));
    assert!(buffer_content.contains(&event.organization));
    assert!(buffer_content.contains("Description"));
    assert!(buffer_content.contains("Highlights"));
    assert!(buffer_content.contains("Technologies"));
}

#[test]
fn test_ui_empty_timeline_rendering() {
    // Setup test terminal
    let backend = TestBackend::new(80, 30);
    let mut terminal = Terminal::new(backend).unwrap();
    let mut app = App::new();
    
    // Set timeline mode
    app.display_mode = DisplayMode::Timeline;
    
    // Ensure timeline is empty
    app.timeline_events.clear();
    
    // Basic test to ensure UI rendering doesn't panic
    terminal.draw(|f| {
        ui::render(f, &mut app);
    }).unwrap();
    
    // Get the buffer to check contents
    let buffer = terminal.backend().buffer().clone();
    
    // Check that the empty timeline message is rendered
    let buffer_content = buffer_to_string(&buffer);
    assert!(buffer_content.contains("Career Timeline"));
    assert!(buffer_content.contains("No timeline events found"));
}

// Helper function to convert buffer to string for checking content
fn buffer_to_string(buffer: &Buffer) -> String {
    let mut result = String::new();
    for row in 0..buffer.area.height {
        for col in 0..buffer.area.width {
            // Use [] operator instead of get method (which is deprecated)
            result.push(buffer[(col, row)].symbol().chars().next().unwrap_or(' '));
        }
        result.push('\n');
    }
    result
}