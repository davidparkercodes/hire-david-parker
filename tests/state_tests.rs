mod common;

use hiredavidparker::tui::state::App;
use hiredavidparker::tui::models::{DisplayMode, TimelineFilter};

#[test]
fn test_app_initialization() {
    // Create a new App instance
    let app = App::new();
    
    // Test initial state values
    assert_eq!(app.menu_index, 0);
    assert_eq!(app.link_index, 0);
    assert_eq!(app.skill_category_index, 0);
    assert_eq!(app.display_mode, DisplayMode::Menu);
    assert_eq!(app.previous_mode, DisplayMode::Menu);
    assert_eq!(app.timeline_filter, TimelineFilter::All);
    assert_eq!(app.timeline_event_index, 0);
    assert_eq!(app.timeline_detail_view, false);
    assert_eq!(app.should_exit, false);
    
    // Test that content is loaded
    assert!(!app.about_content.is_empty());
    assert!(!app.skills_content.is_empty());
    assert!(!app.projects_content.is_empty());
    assert!(!app.why_warp_content.is_empty());
    assert!(!app.welcome_content.is_empty());
    assert!(!app.timeline_content.is_empty());
    
    // Test that timeline events are loaded and sorted
    assert!(!app.timeline_events.is_empty());
    
    // Test that timeline events are sorted by year
    let years: Vec<u16> = app.timeline_events.iter().map(|e| e.year).collect();
    let mut sorted_years = years.clone();
    sorted_years.sort();
    assert_eq!(years, sorted_years);
}

#[test]
fn test_filtered_events() {
    let app = App::new();
    
    // Get filtered events
    let filtered_events = app.get_filtered_events();
    
    // Test that filtered events returns all events for now
    assert_eq!(filtered_events.len(), app.timeline_events.len());
}