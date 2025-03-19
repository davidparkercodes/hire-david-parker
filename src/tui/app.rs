use crate::{about, skills, projects, why_warp, welcome, timeline, load_timeline_data, TimelineEvent};
use crossterm::{
    event::{self, DisableMouseCapture, EnableMouseCapture, KeyCode, KeyEventKind},
    execute,
    terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen},
};
use ratatui::prelude::*;
use serde::{Deserialize, Serialize};
use std::{
    error::Error,
    fs,
    io,
    time::Duration,
};

use super::ui;
use super::event::{Event as AppEvent, EventHandler};

/// Skill data structure for visualization
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Skill {
    pub name: String,
    pub level: u8,
}

/// Skill category structure
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SkillCategory {
    pub name: String,
    pub skills: Vec<Skill>,
}

/// Complete skills data
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SkillsData {
    pub categories: Vec<SkillCategory>,
}

/// Timeline event type - kept for compatibility but no longer used for filtering
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum TimelineType {
    #[serde(rename = "career")]
    Career,
    #[serde(rename = "education")]
    Education,
    #[serde(rename = "certification")]
    Certification,
    #[serde(rename = "project")]
    Project,
    #[serde(other)]
    Other,
}

/// Timeline event structure
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TimelineEvent {
    /// Year of event
    pub year: u16,
    /// Type of timeline event
    #[serde(rename = "type")]
    pub event_type: TimelineType,
    /// Event title
    pub title: String,
    /// Organization name (company, school, etc.)
    pub organization: String,
    /// Event description
    pub description: String,
    /// Key achievements or highlights
    pub highlights: Option<Vec<String>>,
    /// Technologies used
    pub technologies: Option<Vec<String>>,
}

/// Timeline filter type - kept as a placeholder but now only has one option
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum TimelineFilter {
    /// Show all events
    All,
}

/// Complete timeline data
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TimelineData {
    /// All timeline events in chronological order
    pub timeline: Vec<TimelineEvent>,
}

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
    /// Current selected link index when in ProjectLinks mode
    pub link_index: usize,
    /// Current selected skill category index when in SkillsVisual mode
    pub skill_category_index: usize,
    /// Current display mode
    pub display_mode: DisplayMode,
    /// Current timeline filter
    pub timeline_filter: TimelineFilter,
    /// Current selected timeline event index
    pub timeline_event_index: usize,
    /// View detailed event info
    pub timeline_detail_view: bool,
    /// About content
    pub about_content: String,
    /// Skills content
    pub skills_content: String,
    /// Skills data for visualization
    pub skills_data: SkillsData,
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
    /// Interactive skills visualization
    SkillsVisual,
    /// Projects section
    Projects,
    /// Project links navigation
    ProjectLinks,
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
            link_index: 0,
            skill_category_index: 0,
            display_mode: DisplayMode::Menu,
            timeline_filter: TimelineFilter::All,
            timeline_event_index: 0,
            timeline_detail_view: false,
            about_content: about(),
            skills_content: skills(),
            skills_data,
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

        // Fix timeline event index if it's out of bounds
        if self.display_mode == DisplayMode::Timeline || self.display_mode == DisplayMode::TimelineDetail {
            let filtered_events = self.get_filtered_events();
            if !filtered_events.is_empty() && self.timeline_event_index >= filtered_events.len() {
                self.timeline_event_index = 0;
            }
        }

        match self.display_mode {
            DisplayMode::Menu => self.handle_menu_keys(key),
            DisplayMode::Timeline => self.handle_timeline_keys(key),
            _ => self.handle_content_keys(key),
        }
    }
    
    /// Get timeline events (no longer filtered)
    pub fn get_filtered_events(&self) -> Vec<&TimelineEvent> {
        // Return all timeline events 
        self.timeline_data.timeline.iter().collect()
    }

    /// Handle keys in timeline detail view mode
    fn handle_timeline_detail_keys(&mut self, key: event::KeyEvent) {
        match key.code {
            KeyCode::Char('q') => {
                self.should_exit = true;
            }
            KeyCode::Esc | KeyCode::Backspace => {
                self.timeline_detail_view = false;
                self.display_mode = DisplayMode::Timeline;
            }
            KeyCode::Left | KeyCode::Char('h') => {
                // Navigate to previous event while staying in detail view
                if self.timeline_event_index > 0 {
                    self.timeline_event_index -= 1;
                }
            }
            KeyCode::Right | KeyCode::Char('l') => {
                // Navigate to next event while staying in detail view
                let filtered_events = self.get_filtered_events();
                
                if !filtered_events.is_empty() && self.timeline_event_index < filtered_events.len() - 1 {
                    self.timeline_event_index += 1;
                }
            }
            _ => {}
        }
    }
    
    /// Handle keys in timeline mode
    fn handle_timeline_keys(&mut self, key: event::KeyEvent) {
        match key.code {
            KeyCode::Char('q') => {
                self.should_exit = true;
            }
            KeyCode::Esc | KeyCode::Backspace => {
                self.display_mode = DisplayMode::Menu;
            }
            KeyCode::Left | KeyCode::Char('h') => {
                if self.timeline_event_index > 0 {
                    self.timeline_event_index -= 1;
                }
            }
            KeyCode::Right | KeyCode::Char('l') => {
                let filtered_events = self.get_filtered_events();
                
                if !filtered_events.is_empty() && self.timeline_event_index < filtered_events.len() - 1 {
                    self.timeline_event_index += 1;
                }
            }
            KeyCode::Enter => {
                // View detail of current timeline event if we have events
                let filtered_events = self.get_filtered_events();
                if !filtered_events.is_empty() {
                    self.timeline_detail_view = true;
                    self.display_mode = DisplayMode::TimelineDetail;
                }
            }
            _ => {}
        }
    }
    
    /// Handle keys in skills visualization mode
    fn handle_skills_visual_keys(&mut self, key: event::KeyEvent) {
        match key.code {
            KeyCode::Char('q') => {
                self.should_exit = true;
            }
            KeyCode::Esc => {
                self.display_mode = DisplayMode::Skills;
                self.skill_category_index = 0;
            }
            KeyCode::Left | KeyCode::Char('h') => {
                // If at first category, go back to Skills view
                if self.skill_category_index == 0 {
                    self.display_mode = DisplayMode::Skills;
                } else {
                    // Otherwise navigate to previous category
                    self.skill_category_index -= 1;
                }
            }
            KeyCode::Right | KeyCode::Char('l') => {
                if !self.skills_data.categories.is_empty() && 
                   self.skill_category_index < self.skills_data.categories.len() - 1 {
                    self.skill_category_index += 1;
                }
            }
            // Keep up/down for compatibility, but they scroll through skills rather than categories
            KeyCode::Up | KeyCode::Char('k') => {
                // Reserved for future skill selection within a category
            }
            KeyCode::Down | KeyCode::Char('j') => {
                // Reserved for future skill selection within a category
            }
            _ => {}
        }
    }
    
    /// Handle keys in project links mode
    fn handle_project_links_keys(&mut self, key: event::KeyEvent) {
        match key.code {
            KeyCode::Char('q') => {
                self.should_exit = true;
            }
            KeyCode::Esc | KeyCode::Left | KeyCode::Char('h') => {
                self.display_mode = DisplayMode::Projects;
                self.link_index = 0;
            }
            KeyCode::Up | KeyCode::Char('k') => {
                if self.link_index > 0 {
                    self.link_index -= 1;
                }
            }
            KeyCode::Down | KeyCode::Char('j') => {
                if !self.links.is_empty() && self.link_index < self.links.len() - 1 {
                    self.link_index += 1;
                }
            }
            KeyCode::Enter => {
                if !self.links.is_empty() && self.link_index < self.links.len() {
                    // Open the selected link
                    if let Err(e) = open::that(&self.links[self.link_index].url) {
                        eprintln!("Failed to open URL: {}", e);
                    }
                }
            }
            _ => {}
        }
    }
    
    /// Handles mouse events - no longer used for hyperlinks
    pub fn handle_mouse_event(&mut self, _mouse: event::MouseEvent) {
        // Mouse events are no longer used for hyperlinks
        // We've switched to keyboard navigation for better accessibility
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
            KeyCode::Right | KeyCode::Char('l') => {
                // For the projects section, allow moving right to see links
                if self.display_mode == DisplayMode::Projects && !self.links.is_empty() {
                    self.display_mode = DisplayMode::ProjectLinks;
                    self.link_index = 0;
                }
                // For the skills section, allow moving right to see skill visualization
                else if self.display_mode == DisplayMode::Skills {
                    self.display_mode = DisplayMode::SkillsVisual;
                    self.skill_category_index = 0;
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