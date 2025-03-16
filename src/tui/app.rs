use crate::{about, skills, projects, why_warp, welcome};
use crossterm::{
    event::{self, DisableMouseCapture, EnableMouseCapture, KeyCode, KeyEventKind, MouseEventKind, MouseButton},
    execute,
    terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen},
};
use ratatui::prelude::*;
use std::{
    error::Error,
    io,
    time::Duration,
};

use super::ui;
use super::event::{Event as AppEvent, EventHandler};

/// Hyperlink information
#[derive(Debug, Clone)]
pub struct Link {
    /// Text of the link
    pub text: String,
    /// URL to open
    pub url: String,
    /// Line position
    pub line: usize,
    /// Start column position
    pub start_column: usize,
    /// End column position
    pub end_column: usize,
}

/// Application state
pub struct App {
    /// Current menu index
    pub menu_index: usize,
    /// Current display mode
    pub display_mode: DisplayMode,
    /// About content
    pub about_content: String,
    /// Skills content
    pub skills_content: String,
    /// Projects content
    pub projects_content: String,
    /// Why Warp content
    pub why_warp_content: String,
    /// Welcome content
    pub welcome_content: String,
    /// Hyperlinks in current view
    pub links: Vec<Link>,
    /// Should the application exit
    pub should_exit: bool,
}

/// Display modes for the application
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum DisplayMode {
    /// Main menu
    Menu,
    /// About me section
    About,
    /// Skills section
    Skills,
    /// Projects section
    Projects,
    /// Why Warp section
    WhyWarp,
}

impl App {
    /// Creates a new app instance
    pub fn new() -> Self {
        Self {
            menu_index: 0,
            display_mode: DisplayMode::Menu,
            about_content: about(),
            skills_content: skills(),
            projects_content: projects(),
            why_warp_content: why_warp(),
            welcome_content: welcome(),
            links: Vec::new(),
            should_exit: false,
        }
    }

    /// Handles key events
    pub fn handle_key_event(&mut self, key: event::KeyEvent) {
        if key.kind != KeyEventKind::Press {
            return;
        }

        match self.display_mode {
            DisplayMode::Menu => self.handle_menu_keys(key),
            _ => self.handle_content_keys(key),
        }
    }
    
    /// Handles mouse events
    pub fn handle_mouse_event(&mut self, mouse: event::MouseEvent) {
        if let MouseEventKind::Down(MouseButton::Left) = mouse.kind {
            // Check if click is on a hyperlink
            for link in &self.links {
                // Calculate approximate position based on text layout
                // This is simplified - actual implementation would need to consider text wrapping
                if mouse.row as usize == link.line &&
                   mouse.column as usize >= link.start_column &&
                   mouse.column as usize <= link.end_column {
                    // Open the URL in default browser
                    if let Err(e) = open::that(&link.url) {
                        eprintln!("Failed to open URL: {}", e);
                    }
                    break;
                }
            }
        }
    }

    /// Handles keys in menu mode
    fn handle_menu_keys(&mut self, key: event::KeyEvent) {
        match key.code {
            KeyCode::Char('q') | KeyCode::Esc => {
                self.should_exit = true;
            }
            KeyCode::Up | KeyCode::Char('k') => {
                if self.menu_index > 0 {
                    self.menu_index -= 1;
                }
            }
            KeyCode::Down | KeyCode::Char('j') => {
                if self.menu_index < 3 {
                    self.menu_index += 1;
                }
            }
            KeyCode::Enter => {
                match self.menu_index {
                    0 => self.display_mode = DisplayMode::About,
                    1 => self.display_mode = DisplayMode::Skills,
                    2 => self.display_mode = DisplayMode::Projects,
                    3 => self.display_mode = DisplayMode::WhyWarp,
                    _ => {}
                }
            }
            _ => {}
        }
    }

    /// Handles keys in content display modes
    fn handle_content_keys(&mut self, key: event::KeyEvent) {
        match key.code {
            KeyCode::Char('q') => {
                self.should_exit = true;
            }
            KeyCode::Esc | KeyCode::Backspace => {
                self.display_mode = DisplayMode::Menu;
            }
            KeyCode::Up | KeyCode::Char('k') => {
                if self.menu_index > 0 {
                    self.menu_index -= 1;
                }
            }
            KeyCode::Down | KeyCode::Char('j') => {
                if self.menu_index < 3 {
                    self.menu_index += 1;
                }
            }
            KeyCode::Enter => {
                match self.menu_index {
                    0 => self.display_mode = DisplayMode::About,
                    1 => self.display_mode = DisplayMode::Skills,
                    2 => self.display_mode = DisplayMode::Projects,
                    3 => self.display_mode = DisplayMode::WhyWarp,
                    _ => {}
                }
            }
            _ => {}
        }
    }
}

/// Runs the TUI application
pub fn run() -> Result<(), Box<dyn Error>> {
    // Setup terminal
    enable_raw_mode()?;
    let mut stdout = io::stdout();
    execute!(stdout, EnterAlternateScreen, EnableMouseCapture)?;
    let backend = CrosstermBackend::new(stdout);
    let mut terminal = Terminal::new(backend)?;

    // Create app instance
    let mut app = App::new();
    
    // Create event handler
    let event_handler = EventHandler::new(Duration::from_millis(100));

    // Main event loop
    loop {
        // Draw UI
        terminal.draw(|f| ui::render(f, &mut app))?;
        
        // Handle events
        if let Ok(event) = event_handler.receiver.recv() {
            match event {
                AppEvent::Key(key) => {
                    app.handle_key_event(key);
                }
                AppEvent::Mouse(mouse) => {
                    app.handle_mouse_event(mouse);
                }
                AppEvent::Tick => {
                    // Update app state if needed
                }
                _ => {}
            }
        }
        
        // Check if we should exit
        if app.should_exit {
            break;
        }
    }

    // Clean up terminal
    disable_raw_mode()?;
    execute!(
        terminal.backend_mut(),
        LeaveAlternateScreen,
        DisableMouseCapture
    )?;
    terminal.show_cursor()?;

    Ok(())
}