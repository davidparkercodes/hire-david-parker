use crate::{about, skills, projects, why_warp, welcome, timeline, load_timeline_data, TimelineEvent};
use crossterm::{
    event::{self, DisableMouseCapture, EnableMouseCapture, KeyCode, KeyEventKind},
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
    /// Timeline content
    pub timeline_content: String,
    /// Timeline events data
    pub timeline_events: Vec<TimelineEvent>,
    /// Currently selected timeline event index
    pub timeline_index: usize,
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
    /// Timeline section
    Timeline,
}

impl App {
    /// Creates a new app instance
    pub fn new() -> Self {
        // Load timeline events
        let mut timeline_events = load_timeline_data().unwrap_or_default();
        
        // Sort events in chronological order (oldest to newest)
        timeline_events.sort_by_key(|event| event.year);
        
        // Initialize with the oldest event (first in chronological order)
        let timeline_index = 0;
        
        Self {
            menu_index: 0,
            display_mode: DisplayMode::Menu,
            about_content: about(),
            skills_content: skills(),
            projects_content: projects(),
            why_warp_content: why_warp(),
            welcome_content: welcome(),
            timeline_content: timeline(),
            timeline_events,
            timeline_index,
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
            DisplayMode::Timeline => self.handle_timeline_keys(key),
            _ => self.handle_content_keys(key),
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
                if self.menu_index < 4 { // Updated to include Timeline
                    self.menu_index += 1;
                }
            }
            KeyCode::Enter => {
                match self.menu_index {
                    0 => self.display_mode = DisplayMode::About,
                    1 => self.display_mode = DisplayMode::Skills,
                    2 => self.display_mode = DisplayMode::Projects,
                    3 => self.display_mode = DisplayMode::WhyWarp,
                    4 => self.display_mode = DisplayMode::Timeline,
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
                if self.menu_index < 4 { // Updated to include Timeline
                    self.menu_index += 1;
                }
            }
            KeyCode::Enter => {
                match self.menu_index {
                    0 => self.display_mode = DisplayMode::About,
                    1 => self.display_mode = DisplayMode::Skills,
                    2 => self.display_mode = DisplayMode::Projects,
                    3 => self.display_mode = DisplayMode::WhyWarp,
                    4 => self.display_mode = DisplayMode::Timeline,
                    _ => {}
                }
            }
            _ => {}
        }
    }
    
    /// Handles keys in timeline display mode
    fn handle_timeline_keys(&mut self, key: event::KeyEvent) {
        match key.code {
            KeyCode::Char('q') => {
                self.should_exit = true;
            }
            KeyCode::Esc | KeyCode::Backspace => {
                self.display_mode = DisplayMode::Menu;
            }
            // Left arrow goes back in time (previous events)
            KeyCode::Left | KeyCode::Char('h') => {
                if self.timeline_index > 0 {
                    self.timeline_index -= 1;
                }
            }
            // Right arrow goes forward in time (later events)
            KeyCode::Right | KeyCode::Char('l') => {
                if !self.timeline_events.is_empty() && self.timeline_index < self.timeline_events.len() - 1 {
                    self.timeline_index += 1;
                }
            }
            KeyCode::Up | KeyCode::Char('k') => {
                if self.menu_index > 0 {
                    self.menu_index -= 1;
                }
            }
            KeyCode::Down | KeyCode::Char('j') => {
                if self.menu_index < 4 {
                    self.menu_index += 1;
                }
            }
            KeyCode::Enter => {
                match self.menu_index {
                    0 => self.display_mode = DisplayMode::About,
                    1 => self.display_mode = DisplayMode::Skills,
                    2 => self.display_mode = DisplayMode::Projects,
                    3 => self.display_mode = DisplayMode::WhyWarp,
                    4 => self.display_mode = DisplayMode::Timeline,
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
        terminal.draw(|f| ui::render(f, &app))?;
        
        // Handle events
        if let Ok(event) = event_handler.receiver.recv() {
            match event {
                AppEvent::Key(key) => {
                    app.handle_key_event(key);
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