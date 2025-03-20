use crossterm::{execute, terminal, event::KeyEvent};
use ratatui::prelude::*;
use std::{error::Error, io, time::Duration, sync::mpsc, panic};
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
    // Set up a panic hook to properly restore terminal
    let original_hook = panic::take_hook();
    panic::set_hook(Box::new(move |panic_info| {
        // Clean up terminal
        let _ = terminal::disable_raw_mode();
        let mut stdout = io::stdout();
        let _ = execute!(stdout, terminal::LeaveAlternateScreen);
        
        // Call the original hook
        original_hook(panic_info);
    }));

    // Set up signal handlers for SIGINT (Ctrl+C)
    #[cfg(unix)]
    {
        use std::sync::atomic::{AtomicBool, Ordering};
        use signal_hook::{iterator::Signals, consts::SIGINT};
        
        static INTERRUPTED: AtomicBool = AtomicBool::new(false);
        
        if let Ok(mut signals) = Signals::new(&[SIGINT]) {
            std::thread::spawn(move || {
                for _ in signals.forever() {
                    INTERRUPTED.store(true, Ordering::SeqCst);
                }
            });
        }
    }

    terminal::enable_raw_mode()?;
    let mut stdout = io::stdout();
    execute!(stdout, terminal::EnterAlternateScreen)?;
    let backend = CrosstermBackend::new(stdout);
    let mut terminal = Terminal::new(backend)?;

    let mut app = App::new();
    
    let event_handler = EventHandler::new(Duration::from_millis(100));

    run_app(&mut terminal, &mut app, &event_handler)?;

    // Clean up
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
    #[cfg(unix)]
    let check_interrupted = || {
        use std::sync::atomic::{AtomicBool, Ordering};
        static INTERRUPTED: AtomicBool = AtomicBool::new(false);
        INTERRUPTED.load(Ordering::SeqCst)
    };

    #[cfg(not(unix))]
    let check_interrupted = || false;

    loop {
        terminal.draw(|f| ui::render(f, app))?;
        
        // Check for SIGINT (Ctrl+C) on Unix platforms
        if check_interrupted() {
            app.should_exit = true;
        }
        
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