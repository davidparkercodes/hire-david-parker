use crate::{about, skills, projects, why_warp, welcome};
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
    /// Previous display mode (for transitions)
    pub previous_mode: DisplayMode,
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
    /// Should the application exit
    pub should_exit: bool,
    /// Transition state
    pub transition: TransitionState,
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

/// Transition states for animations
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum TransitionState {
    /// No transition is occurring
    None,
    /// Wipe transition is in progress
    Wipe { 
        /// Progress from 0-100
        progress: u8, 
        /// Direction of the wipe
        direction: WipeDirection,
    },
}

/// Direction for wipe transitions
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum WipeDirection {
    /// Wipe from left to right
    LeftToRight,
    /// Wipe from right to left
    RightToLeft,
}

impl App {
    /// Creates a new app instance
    pub fn new() -> Self {
        Self {
            menu_index: 0,
            display_mode: DisplayMode::Menu,
            previous_mode: DisplayMode::Menu,
            about_content: about(),
            skills_content: skills(),
            projects_content: projects(),
            why_warp_content: why_warp(),
            welcome_content: welcome(),
            should_exit: false,
            transition: TransitionState::None,
        }
    }
    
    /// Starts a transition to a new display mode
    pub fn transition_to(&mut self, mode: DisplayMode) {
        // Only start a transition if we're not already in one
        if matches!(self.transition, TransitionState::None) {
            // Set the direction based on menu position
            let direction = match (self.display_mode, mode) {
                (DisplayMode::Menu, _) => WipeDirection::LeftToRight,
                (_, DisplayMode::Menu) => WipeDirection::RightToLeft,
                (_, _) => {
                    // When transitioning between content pages, use the menu index to determine direction
                    let current_index = self.mode_to_index(self.display_mode);
                    let target_index = self.mode_to_index(mode);
                    
                    if target_index > current_index {
                        WipeDirection::LeftToRight
                    } else {
                        WipeDirection::RightToLeft
                    }
                }
            };
            
            self.previous_mode = self.display_mode;
            self.display_mode = mode;
            self.transition = TransitionState::Wipe {
                progress: 0,
                direction,
            };
        }
    }
    
    /// Convert DisplayMode to menu index
    fn mode_to_index(&self, mode: DisplayMode) -> usize {
        match mode {
            DisplayMode::Menu => 0,
            DisplayMode::About => 0,
            DisplayMode::Skills => 1,
            DisplayMode::Projects => 2,
            DisplayMode::WhyWarp => 3,
        }
    }
    
    /// Updates the transition state
    pub fn update_transition(&mut self) {
        match self.transition {
            TransitionState::Wipe { progress, direction } => {
                if progress >= 100 {
                    // Transition complete
                    self.transition = TransitionState::None;
                } else {
                    // Increment progress by a step amount
                    // Use a smaller step size for smoother animations
                    const STEP: u8 = 5;
                    let new_progress = (progress + STEP).min(100);
                    self.transition = TransitionState::Wipe {
                        progress: new_progress,
                        direction,
                    };
                }
            }
            TransitionState::None => {}
        }
    }

    /// Handles key events
    pub fn handle_key_event(&mut self, key: event::KeyEvent) {
        if key.kind != KeyEventKind::Press {
            return;
        }

        // Only handle key events if not in a transition
        if matches!(self.transition, TransitionState::None) {
            match self.display_mode {
                DisplayMode::Menu => self.handle_menu_keys(key),
                _ => self.handle_content_keys(key),
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
                    0 => self.transition_to(DisplayMode::About),
                    1 => self.transition_to(DisplayMode::Skills),
                    2 => self.transition_to(DisplayMode::Projects),
                    3 => self.transition_to(DisplayMode::WhyWarp),
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
                self.transition_to(DisplayMode::Menu);
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
                    0 => self.transition_to(DisplayMode::About),
                    1 => self.transition_to(DisplayMode::Skills),
                    2 => self.transition_to(DisplayMode::Projects),
                    3 => self.transition_to(DisplayMode::WhyWarp),
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
    
    // Create event handler with faster tick rate for smoother animations
    let event_handler = EventHandler::new(Duration::from_millis(50));

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
                    // Update transition animation if active
                    if !matches!(app.transition, TransitionState::None) {
                        app.update_transition();
                    }
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