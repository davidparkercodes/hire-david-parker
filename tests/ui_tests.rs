mod common;

use hiredavidparker::tui::state::App;
use hiredavidparker::tui::models::{DisplayMode, TimelineEvent, SkillCategory, Skill};
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

// Helper function to convert buffer to string for checking content
fn buffer_to_string(buffer: &Buffer) -> String {
    let mut result = String::new();
    for row in 0..buffer.area.height {
        for col in 0..buffer.area.width {
            result.push(buffer.get(col, row).symbol.chars().next().unwrap_or(' '));
        }
        result.push('\n');
    }
    result
}