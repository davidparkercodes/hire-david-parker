use crossterm::{execute, terminal};
use ratatui::prelude::*;
use std::{error::Error, io, time::Duration, sync::mpsc};
use super::{ui, event::{Event as AppEvent, EventHandler}, state::App};

// Define a trait for event handlers to allow for mocking in tests
pub trait EventHandlerTrait {
    fn receiver(&self) -> &mpsc::Receiver<AppEvent>;
}

// Implement the trait for the real EventHandler
impl EventHandlerTrait for EventHandler {
    fn receiver(&self) -> &mpsc::Receiver<AppEvent> {
        &self.receiver
    }
}

#[cfg(not(test))]
pub fn run() -> Result<(), Box<dyn Error>> {
    terminal::enable_raw_mode()?;
    let mut stdout = io::stdout();
    execute!(stdout, terminal::EnterAlternateScreen)?;
    let backend = CrosstermBackend::new(stdout);
    let mut terminal = Terminal::new(backend)?;

    let mut app = App::new();
    
    let event_handler = EventHandler::new(Duration::from_millis(100));

    run_app(&mut terminal, &mut app, &event_handler)?;

    terminal::disable_raw_mode()?;
    execute!(
        terminal.backend_mut(),
        terminal::LeaveAlternateScreen
    )?;
    terminal.show_cursor()?;

    Ok(())
}

// Extracted the main application loop to a separate function for better testability
// Now uses a trait object to allow for mocking in tests
pub fn run_app<B: Backend, H: EventHandlerTrait>(
    terminal: &mut Terminal<B>,
    app: &mut App,
    event_handler: &H,
) -> Result<(), Box<dyn Error>> {
    loop {
        terminal.draw(|f| ui::render(f, app))?;
        
        if let Ok(event) = event_handler.receiver().recv() {
            match event {
                AppEvent::Key(key) => {
                    app.handle_key_event(key);
                }
                AppEvent::Tick => {}
                _ => {}
            }
        }
        
        if app.should_exit {
            break;
        }
    }

    Ok(())
}

// Test-friendly version for use in tests
#[cfg(test)]
pub fn run() -> Result<(), Box<dyn Error>> {
    // In test mode, just return OK without interacting with the terminal
    Ok(())
}