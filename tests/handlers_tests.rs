mod common;

use crossterm::event::{KeyCode, KeyEvent, KeyEventKind, KeyModifiers};
use hiredavidparker::tui::state::App;
use hiredavidparker::tui::models::DisplayMode;

// Helper function to create a keyboard event
fn create_key_event(code: KeyCode) -> KeyEvent {
    KeyEvent {
        code,
        modifiers: KeyModifiers::NONE,
        kind: KeyEventKind::Press,
        state: crossterm::event::KeyEventState::NONE,
    }
}

#[test]
fn test_menu_key_handling() {
    // Create a new App instance
    let mut app = App::new();
    
    // Test navigation in menu mode
    app.display_mode = DisplayMode::Menu;
    
    // Test down key navigation
    app.handle_key_event(create_key_event(KeyCode::Down));
    assert_eq!(app.menu_index, 1);
    app.handle_key_event(create_key_event(KeyCode::Char('j')));
    assert_eq!(app.menu_index, 2);
    
    // Test up key navigation
    app.handle_key_event(create_key_event(KeyCode::Up));
    assert_eq!(app.menu_index, 1);
    app.handle_key_event(create_key_event(KeyCode::Char('k')));
    assert_eq!(app.menu_index, 0);
    
    // Test selecting an option
    app.handle_key_event(create_key_event(KeyCode::Enter));
    assert_eq!(app.display_mode, DisplayMode::About);
    
    // Test exit with q
    app.display_mode = DisplayMode::Menu;
    app.handle_key_event(create_key_event(KeyCode::Char('q')));
    assert!(app.should_exit);
}

#[test]
fn test_content_navigation() {
    // Create a new App instance
    let mut app = App::new();
    
    // Test About mode navigation
    app.display_mode = DisplayMode::About;
    
    // Test escape back to menu
    app.handle_key_event(create_key_event(KeyCode::Esc));
    assert_eq!(app.display_mode, DisplayMode::Menu);
    
    // Test Skills mode and Skills visual mode
    app.display_mode = DisplayMode::Skills;
    app.handle_key_event(create_key_event(KeyCode::Right));
    assert_eq!(app.display_mode, DisplayMode::SkillsVisual);
    
    // Test escape from Skills visual mode back to Skills
    app.handle_key_event(create_key_event(KeyCode::Esc));
    assert_eq!(app.display_mode, DisplayMode::Skills);
    
    // Test Projects mode and Project links mode
    app.display_mode = DisplayMode::Projects;
    app.handle_key_event(create_key_event(KeyCode::Right));
    assert_eq!(app.display_mode, DisplayMode::ProjectLinks);
    
    // Test escape from Project links mode back to Projects
    app.handle_key_event(create_key_event(KeyCode::Esc));
    assert_eq!(app.display_mode, DisplayMode::Projects);
}

#[test]
fn test_timeline_navigation() {
    // Create a new App instance
    let mut app = App::new();
    
    // Check that we have timeline events to test with
    assert!(!app.timeline_events.is_empty());
    
    // Test Timeline mode
    app.display_mode = DisplayMode::Timeline;
    
    // Test right key navigation in timeline
    let initial_index = app.timeline_index;
    app.handle_key_event(create_key_event(KeyCode::Right));
    assert_eq!(app.timeline_index, initial_index + 1);
    
    // Test entering timeline detail view
    app.handle_key_event(create_key_event(KeyCode::Enter));
    assert!(app.timeline_detail_view);
    
    // Test escape from timeline detail view
    app.handle_key_event(create_key_event(KeyCode::Esc));
    assert!(!app.timeline_detail_view);
}

#[test]
fn test_non_press_key_events_are_ignored() {
    // Create a new App instance
    let mut app = App::new();
    
    // Create a key event that is not a press
    let release_event = KeyEvent {
        code: KeyCode::Char('q'),
        modifiers: KeyModifiers::NONE,
        kind: KeyEventKind::Release,
        state: crossterm::event::KeyEventState::NONE,
    };
    
    // Test that the release event is ignored
    app.handle_key_event(release_event);
    assert!(!app.should_exit);
}