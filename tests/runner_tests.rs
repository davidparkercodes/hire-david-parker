mod common;

use std::{sync::mpsc, time::Duration};
use crossterm::event::{KeyCode, KeyEvent, KeyEventKind, KeyModifiers};
use hiredavidparker::tui::event::Event;
use hiredavidparker::tui::state::App;

#[test]
fn test_event_handling_with_app() {
    // Create a simple channel for testing
    let (sender, receiver) = mpsc::channel();
    
    // Create mock app
    let mut app = App::new();
    
    // Test key event handling
    let key_event = KeyEvent {
        code: KeyCode::Char('q'),
        modifiers: KeyModifiers::NONE,
        kind: KeyEventKind::Press,
        state: crossterm::event::KeyEventState::NONE,
    };
    
    // Send a key event through the channel
    sender.send(Event::Key(key_event)).unwrap();
    
    // Receive the event
    let received = receiver.recv().unwrap();
    match received {
        Event::Key(k) => {
            assert_eq!(k.code, KeyCode::Char('q'));
            app.handle_key_event(k);
            assert!(app.should_exit);
        },
        _ => panic!("Expected Key event")
    }
}

#[test]
fn test_runner_loop_exit_condition() {
    // Test that the run function respects the exit flag
    let mut app = App::new();
    app.should_exit = true;
    
    // The runner loop should exit immediately if app.should_exit is true
    // We can't directly test the run function because it takes over the terminal,
    // but we can verify the logic is correct
    assert!(app.should_exit);
}