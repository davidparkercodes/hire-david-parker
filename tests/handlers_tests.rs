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
    
    // Test exit with Esc
    app.should_exit = false;
    app.handle_key_event(create_key_event(KeyCode::Esc));
    assert!(app.should_exit);
    
    // Test menu bounds
    app.should_exit = false;
    app.menu_index = 0;
    app.handle_key_event(create_key_event(KeyCode::Up)); // Should not go below 0
    assert_eq!(app.menu_index, 0);
    
    app.menu_index = 4;
    app.handle_key_event(create_key_event(KeyCode::Down)); // Should not go above 4
    assert_eq!(app.menu_index, 4);
    
    // Test all menu selection options
    app.menu_index = 1;
    app.handle_key_event(create_key_event(KeyCode::Enter));
    assert_eq!(app.display_mode, DisplayMode::Skills);
    
    app.display_mode = DisplayMode::Menu;
    app.menu_index = 2;
    app.handle_key_event(create_key_event(KeyCode::Enter));
    assert_eq!(app.display_mode, DisplayMode::Projects);
    
    app.display_mode = DisplayMode::Menu;
    app.menu_index = 3;
    app.handle_key_event(create_key_event(KeyCode::Enter));
    assert_eq!(app.display_mode, DisplayMode::WhyWarp);
    
    app.display_mode = DisplayMode::Menu;
    app.menu_index = 4;
    app.handle_key_event(create_key_event(KeyCode::Enter));
    assert_eq!(app.display_mode, DisplayMode::Timeline);
    assert_eq!(app.timeline_index, 0);
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
    
    // Test backspace back to menu
    app.display_mode = DisplayMode::About;
    app.handle_key_event(create_key_event(KeyCode::Backspace));
    assert_eq!(app.display_mode, DisplayMode::Menu);
    
    // Test exit with q
    app.display_mode = DisplayMode::About;
    app.handle_key_event(create_key_event(KeyCode::Char('q')));
    assert!(app.should_exit);
    
    // Test menu navigation in content mode
    app.should_exit = false;
    app.display_mode = DisplayMode::About;
    app.menu_index = 1;
    app.handle_key_event(create_key_event(KeyCode::Up));
    assert_eq!(app.menu_index, 0);
    
    app.display_mode = DisplayMode::About;
    app.menu_index = 0;
    app.handle_key_event(create_key_event(KeyCode::Down));
    assert_eq!(app.menu_index, 1);
    
    // Test menu selection in content mode
    app.display_mode = DisplayMode::About;
    app.menu_index = 1;
    app.handle_key_event(create_key_event(KeyCode::Enter));
    assert_eq!(app.display_mode, DisplayMode::Skills);
    
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
    
    // Test all content Enter menu options
    app.display_mode = DisplayMode::About;
    app.menu_index = 0;
    app.handle_key_event(create_key_event(KeyCode::Enter));
    assert_eq!(app.display_mode, DisplayMode::About);
    
    app.display_mode = DisplayMode::About;
    app.menu_index = 2;
    app.handle_key_event(create_key_event(KeyCode::Enter));
    assert_eq!(app.display_mode, DisplayMode::Projects);
    
    app.display_mode = DisplayMode::About;
    app.menu_index = 3;
    app.handle_key_event(create_key_event(KeyCode::Enter));
    assert_eq!(app.display_mode, DisplayMode::WhyWarp);
    
    app.display_mode = DisplayMode::About;
    app.menu_index = 4;
    app.handle_key_event(create_key_event(KeyCode::Enter));
    assert_eq!(app.display_mode, DisplayMode::Timeline);
}

#[test]
fn test_skills_visual_navigation() {
    let mut app = App::new();
    
    // Set up with initial data
    app.display_mode = DisplayMode::SkillsVisual;
    app.skill_category_index = 1; // Assume we have at least 2 categories
    
    // Test exit
    app.handle_key_event(create_key_event(KeyCode::Char('q')));
    assert!(app.should_exit);
    
    // Test left navigation
    app.should_exit = false;
    app.skill_category_index = 1;
    app.handle_key_event(create_key_event(KeyCode::Left));
    assert_eq!(app.skill_category_index, 0);
    
    // Test left on first category returns to Skills mode
    app.display_mode = DisplayMode::SkillsVisual;
    app.skill_category_index = 0;
    app.handle_key_event(create_key_event(KeyCode::Left));
    assert_eq!(app.display_mode, DisplayMode::Skills);
    
    // Test h key navigation (same as left)
    app.display_mode = DisplayMode::SkillsVisual;
    app.skill_category_index = 1;
    app.handle_key_event(create_key_event(KeyCode::Char('h')));
    assert_eq!(app.skill_category_index, 0);
    
    // Test right navigation (if we have more than one category)
    if app.skills_data.categories.len() > 1 {
        app.display_mode = DisplayMode::SkillsVisual;
        app.skill_category_index = 0;
        app.handle_key_event(create_key_event(KeyCode::Right));
        assert_eq!(app.skill_category_index, 1);
        
        // Test l key navigation (same as right)
        app.skill_category_index = 0;
        app.handle_key_event(create_key_event(KeyCode::Char('l')));
        assert_eq!(app.skill_category_index, 1);
    }
    
    // Test bounds at maximum
    app.display_mode = DisplayMode::SkillsVisual;
    app.skill_category_index = app.skills_data.categories.len() - 1;
    app.handle_key_event(create_key_event(KeyCode::Right));
    assert_eq!(app.skill_category_index, app.skills_data.categories.len() - 1);
}

#[test]
fn test_project_links_navigation() {
    let mut app = App::new();
    
    // Set up with initial data
    app.display_mode = DisplayMode::ProjectLinks;
    app.link_index = 0;
    
    // Test exit
    app.handle_key_event(create_key_event(KeyCode::Char('q')));
    assert!(app.should_exit);
    
    // Test escape returns to Projects mode
    app.should_exit = false;
    app.handle_key_event(create_key_event(KeyCode::Esc));
    assert_eq!(app.display_mode, DisplayMode::Projects);
    
    // Test left key returns to Projects mode
    app.display_mode = DisplayMode::ProjectLinks;
    app.handle_key_event(create_key_event(KeyCode::Left));
    assert_eq!(app.display_mode, DisplayMode::Projects);
    
    // Test h key returns to Projects mode
    app.display_mode = DisplayMode::ProjectLinks;
    app.handle_key_event(create_key_event(KeyCode::Char('h')));
    assert_eq!(app.display_mode, DisplayMode::Projects);
    
    // Test up/down navigation
    app.display_mode = DisplayMode::ProjectLinks;
    app.link_index = 1;
    app.handle_key_event(create_key_event(KeyCode::Up));
    assert_eq!(app.link_index, 0);
    
    app.handle_key_event(create_key_event(KeyCode::Down));
    assert_eq!(app.link_index, 1);
    
    // Test k/j keys for navigation
    app.link_index = 1;
    app.handle_key_event(create_key_event(KeyCode::Char('k')));
    assert_eq!(app.link_index, 0);
    
    app.handle_key_event(create_key_event(KeyCode::Char('j')));
    assert_eq!(app.link_index, 1);
    
    // Test upper bound for link index
    app.link_index = 0;
    app.handle_key_event(create_key_event(KeyCode::Up));
    assert_eq!(app.link_index, 0);
}

#[test]
fn test_timeline_navigation() {
    // Create a new App instance
    let mut app = App::new();
    
    // Check that we have timeline events to test with
    assert!(!app.timeline_events.is_empty());
    
    // Test Timeline mode
    app.display_mode = DisplayMode::Timeline;
    
    // Test exit with q
    app.handle_key_event(create_key_event(KeyCode::Char('q')));
    assert!(app.should_exit);
    
    // Test escape returns to menu
    app.should_exit = false;
    app.handle_key_event(create_key_event(KeyCode::Esc));
    assert_eq!(app.display_mode, DisplayMode::Menu);
    
    // Test left navigation
    app.display_mode = DisplayMode::Timeline;
    app.timeline_index = 1;
    app.handle_key_event(create_key_event(KeyCode::Left));
    assert_eq!(app.timeline_index, 0);
    
    // Test h key navigation (same as left)
    app.timeline_index = 1;
    app.handle_key_event(create_key_event(KeyCode::Char('h')));
    assert_eq!(app.timeline_index, 0);
    
    // Test left at first item returns to menu
    app.timeline_index = 0;
    app.handle_key_event(create_key_event(KeyCode::Left));
    assert_eq!(app.display_mode, DisplayMode::Menu);
    
    // Test right key navigation in timeline
    app.display_mode = DisplayMode::Timeline;
    app.timeline_index = 0;
    app.handle_key_event(create_key_event(KeyCode::Right));
    assert_eq!(app.timeline_index, 1);
    
    // Test l key navigation (same as right)
    app.timeline_index = 0;
    app.handle_key_event(create_key_event(KeyCode::Char('l')));
    assert_eq!(app.timeline_index, 1);
    
    // Test right at max item
    if app.timeline_events.len() > 1 {
        app.timeline_index = app.timeline_events.len() - 1;
        app.handle_key_event(create_key_event(KeyCode::Right));
        assert_eq!(app.timeline_index, app.timeline_events.len() - 1);
    }
    
    // Test entering timeline detail view
    app.timeline_index = 0;
    app.handle_key_event(create_key_event(KeyCode::Enter));
    assert!(app.timeline_detail_view);
}

#[test]
fn test_timeline_detail_navigation() {
    // Create a new App instance
    let mut app = App::new();
    
    // Check that we have timeline events to test with
    assert!(!app.timeline_events.is_empty());
    
    // Set up timeline detail view
    app.display_mode = DisplayMode::Timeline;
    app.timeline_detail_view = true;
    app.timeline_index = 1; // Assume we have at least 2 events
    
    // Test exit with q
    app.handle_key_event(create_key_event(KeyCode::Char('q')));
    assert!(app.should_exit);
    
    // Test escape closes detail view
    app.should_exit = false;
    app.timeline_detail_view = true;
    app.handle_key_event(create_key_event(KeyCode::Esc));
    assert!(!app.timeline_detail_view);
    
    // Test backspace closes detail view
    app.timeline_detail_view = true;
    app.handle_key_event(create_key_event(KeyCode::Backspace));
    assert!(!app.timeline_detail_view);
    
    // Test left navigation
    app.timeline_detail_view = true;
    app.timeline_index = 1;
    app.handle_key_event(create_key_event(KeyCode::Left));
    assert_eq!(app.timeline_index, 0);
    assert_eq!(app.timeline_event_index, 0);
    
    // Test h key navigation (same as left)
    app.timeline_detail_view = true;
    app.timeline_index = 1;
    app.handle_key_event(create_key_event(KeyCode::Char('h')));
    assert_eq!(app.timeline_index, 0);
    assert_eq!(app.timeline_event_index, 0);
    
    // Test left at first item closes detail view
    app.timeline_detail_view = true;
    app.timeline_index = 0;
    app.handle_key_event(create_key_event(KeyCode::Left));
    assert!(!app.timeline_detail_view);
    
    // Test right key navigation
    app.timeline_detail_view = true;
    app.timeline_index = 0;
    app.handle_key_event(create_key_event(KeyCode::Right));
    assert_eq!(app.timeline_index, 1);
    assert_eq!(app.timeline_event_index, 1);
    
    // Test l key navigation (same as right)
    app.timeline_detail_view = true;
    app.timeline_index = 0;
    app.handle_key_event(create_key_event(KeyCode::Char('l')));
    assert_eq!(app.timeline_index, 1);
    assert_eq!(app.timeline_event_index, 1);
    
    // Test right at max item
    if app.timeline_events.len() > 1 {
        app.timeline_detail_view = true;
        app.timeline_index = app.timeline_events.len() - 1;
        app.handle_key_event(create_key_event(KeyCode::Right));
        assert_eq!(app.timeline_index, app.timeline_events.len() - 1);
    }
    
    // Test other key does nothing
    app.timeline_detail_view = true;
    let original_index = app.timeline_index;
    app.handle_key_event(create_key_event(KeyCode::Char('x')));
    assert_eq!(app.timeline_index, original_index);
    assert!(app.timeline_detail_view);
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

#[test]
fn test_handle_key_event_timeline_index_reset() {
    let mut app = App::new();
    
    // Set up the app in a state where timeline_event_index is out of bounds
    app.display_mode = DisplayMode::Timeline;
    let events_count = app.timeline_events.len();
    assert!(events_count > 0, "Test requires at least one timeline event");
    
    app.timeline_event_index = events_count; // One past the last valid index
    
    // Process any key event in the timeline mode
    app.handle_key_event(create_key_event(KeyCode::Char('a')));
    
    // Check that the index has been reset to a valid value
    assert!(app.timeline_event_index < events_count);
}