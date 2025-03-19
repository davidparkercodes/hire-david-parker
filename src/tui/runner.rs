use crossterm::{execute, terminal};
use ratatui::prelude::*;
use std::{error::Error, io, time::Duration};
use super::{ui, event::{Event as AppEvent, EventHandler}, state::App};

pub fn run() -> Result<(), Box<dyn Error>> {
    terminal::enable_raw_mode()?;
    let mut stdout = io::stdout();
    execute!(stdout, terminal::EnterAlternateScreen)?;
    let backend = CrosstermBackend::new(stdout);
    let mut terminal = Terminal::new(backend)?;

    let mut app = App::new();
    
    let event_handler = EventHandler::new(Duration::from_millis(100));

    loop {
        terminal.draw(|f| ui::render(f, &mut app))?;
        
        if let Ok(event) = event_handler.receiver.recv() {
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

    terminal::disable_raw_mode()?;
    execute!(
        terminal.backend_mut(),
        terminal::LeaveAlternateScreen
    )?;
    terminal.show_cursor()?;

    Ok(())
}