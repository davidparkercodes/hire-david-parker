use std::time::Duration;

use crossterm::event::{KeyCode, KeyEvent, KeyModifiers};
use hiredavidparker::tui::event::{Event, EventHandler};

#[test]
fn test_event_handler_creation() {
    let event_handler = EventHandler::new(Duration::from_millis(100));
    assert!(event_handler.receiver.try_recv().is_err());
}

#[test]
fn test_event_enum_debug() {
    // Test debug formatting for Event enum variants
    let tick_event = Event::Tick;
    let key_event = Event::Key(KeyEvent::new(KeyCode::Enter, KeyModifiers::NONE));
    let resize_event = Event::Resize(80, 24);
    
    assert_eq!(format!("{:?}", tick_event), "Tick");
    assert!(format!("{:?}", key_event).contains("Key"));
    assert_eq!(format!("{:?}", resize_event), "Resize(80, 24)");
}

#[test]
fn test_event_handler_debug() {
    let event_handler = EventHandler::new(Duration::from_millis(100));
    let debug_str = format!("{:?}", event_handler);
    assert!(debug_str.contains("EventHandler"));
}

#[test]
fn test_event_clone() {
    let resize_event = Event::Resize(80, 24);
    let cloned_event = resize_event.clone();
    assert_eq!(format!("{:?}", resize_event), format!("{:?}", cloned_event));
}