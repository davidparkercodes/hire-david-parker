use crossterm::event::{self, KeyCode, KeyEventKind};
use std::process::Command;
use super::models::DisplayMode;
use super::state::App;

impl App {
    pub fn handle_key_event(&mut self, key: event::KeyEvent) {
        if key.kind != KeyEventKind::Press {
            return;
        }

        if self.display_mode == DisplayMode::Timeline {
            let filtered_events = self.get_filtered_events();
            if !filtered_events.is_empty() && self.timeline_event_index >= filtered_events.len() {
                self.timeline_event_index = 0;
            }
        }

        match self.display_mode {
            DisplayMode::Menu => self.handle_menu_keys(key),
            DisplayMode::Timeline => {
                if self.timeline_detail_view {
                    self.handle_timeline_detail_keys(key)
                } else {
                    self.handle_timeline_keys(key)
                }
            },
            DisplayMode::SkillsVisual => self.handle_skills_visual_keys(key),
            DisplayMode::ProjectLinks => self.handle_project_links_keys(key),
            _ => self.handle_content_keys(key),
        }
    }
    
    fn switch_to_selected_screen(&mut self) {
        match self.menu_index {
            0 => {
                self.previous_mode = self.display_mode;
                self.display_mode = DisplayMode::About;
            },
            1 => {
                self.previous_mode = self.display_mode;
                self.display_mode = DisplayMode::SkillsVisual;
                self.skill_category_index = 0;
                self.skills_page = 0;
            },
            2 => {
                self.previous_mode = self.display_mode;
                self.display_mode = DisplayMode::Projects;
            },
            3 => {
                self.previous_mode = self.display_mode;
                self.display_mode = DisplayMode::WhyWarp;
            },
            4 => {
                self.previous_mode = self.display_mode;
                self.display_mode = DisplayMode::Timeline;
                
                self.timeline_index = 0;
                self.timeline_event_index = self.timeline_index;
            },
            5 => {
                self.previous_mode = self.display_mode;
                self.display_mode = DisplayMode::Contact;
            },
            _ => {}
        }
    }

    fn handle_timeline_detail_keys(&mut self, key: event::KeyEvent) {
        match key.code {
            KeyCode::Char('q') => {
                self.should_exit = true;
            }
            KeyCode::Esc | KeyCode::Backspace => {
                self.timeline_detail_view = false;
            }
            KeyCode::Left | KeyCode::Char('h') => {
                if self.timeline_index > 0 {
                    self.timeline_index -= 1;
                    self.timeline_event_index = self.timeline_index;
                } else {
                    self.timeline_detail_view = false;
                }
            }
            KeyCode::Right | KeyCode::Char('l') => {
                if !self.timeline_events.is_empty() && self.timeline_index < self.timeline_events.len() - 1 {
                    self.timeline_index += 1;
                    self.timeline_event_index = self.timeline_index;
                }
            }
            _ => {}
        }
    }
    
    fn handle_timeline_keys(&mut self, key: event::KeyEvent) {
        match key.code {
            KeyCode::Char('q') => {
                self.should_exit = true;
            }
            KeyCode::Esc | KeyCode::Backspace => {
                self.previous_mode = DisplayMode::Timeline;
                self.menu_index = 4;
                self.display_mode = DisplayMode::Menu;
                self.timeline_detail_view = false;
            }
            KeyCode::Up | KeyCode::Char('k') => {
                if self.menu_index > 0 {
                    self.menu_index -= 1;
                    self.previous_mode = DisplayMode::Timeline;
                    self.display_mode = DisplayMode::Menu;
                    self.switch_to_selected_screen();
                }
            }
            KeyCode::Down | KeyCode::Char('j') => {
                if self.menu_index < 5 {
                    self.menu_index += 1;
                    self.previous_mode = DisplayMode::Timeline;
                    self.display_mode = DisplayMode::Menu;
                    self.switch_to_selected_screen();
                }
            }
            KeyCode::Left | KeyCode::Char('h') => {
                if self.timeline_index > 0 {
                    self.timeline_index -= 1;
                    self.timeline_event_index = self.timeline_index; 
                } else {
                    // Force menu display instead of automatically going to About
                    let prev_display_mode = self.display_mode;
                    self.menu_index = 4; // Set to Timeline menu item
                    self.previous_mode = prev_display_mode;
                    self.display_mode = DisplayMode::Menu;
                    self.timeline_detail_view = false;
                    
                    // Important: Early return to bypass any post-handler logic that
                    // might cause automatic menu selection (switching to About)
                    return;
                }
            }
            KeyCode::Right | KeyCode::Char('l') => {
                if !self.timeline_events.is_empty() && self.timeline_index < self.timeline_events.len() - 1 {
                    self.timeline_index += 1;
                    self.timeline_event_index = self.timeline_index;
                }
            }
            KeyCode::Enter => {
                if !self.timeline_events.is_empty() {
                    self.timeline_detail_view = true;
                }
            }
            _ => {}
        }
    }
    
    fn handle_skills_visual_keys(&mut self, key: event::KeyEvent) {
        match key.code {
            KeyCode::Char('q') => {
                self.should_exit = true;
            }
            KeyCode::Esc => {
                // Return to menu and set Menu index to skills (1)
                self.previous_mode = self.display_mode;
                self.display_mode = DisplayMode::Menu;
                self.menu_index = 1; // Set to Skills menu item
                self.skill_category_index = 0;
                self.skills_page = 0;
            }
            KeyCode::Up | KeyCode::Char('k') => {
                // First try to navigate categories up
                if self.skill_category_index > 0 {
                    self.skill_category_index -= 1;
                    self.skills_page = 0; // Reset page when changing categories
                } else {
                    // If at first category, go to the About menu item (0) and activate it
                    self.previous_mode = self.display_mode;
                    self.display_mode = DisplayMode::About;
                    self.menu_index = 0; // Set to About menu item
                }
            }
            KeyCode::Down | KeyCode::Char('j') => {
                // First try to navigate categories down
                if !self.skills_data.categories.is_empty() && 
                   self.skill_category_index < self.skills_data.categories.len() - 1 {
                    self.skill_category_index += 1;
                    self.skills_page = 0; // Reset page when changing categories
                } else {
                    // If at last category, go to Projects menu item (2) and activate it
                    self.previous_mode = self.display_mode;
                    self.display_mode = DisplayMode::Projects;
                    self.menu_index = 2; // Set to Projects menu item
                }
            }
            KeyCode::Left | KeyCode::Char('h') => {
                if self.skills_page > 0 {
                    self.skills_page -= 1;
                }
            }
            KeyCode::Right | KeyCode::Char('l') => {
                // Let the UI rendering handle the upper bound check since
                // it depends on the screen size and number of skills
                if !self.skills_data.categories.is_empty() {
                    self.skills_page += 1;
                    // The UI will validate and adjust if this is too high
                }
            }
            // Page navigation now only handled by left/right keys
            _ => {}
        }
    }
    
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
                self.link_index += 1;
            }
            KeyCode::Enter => {
                if let Err(e) = Command::new("open").arg("https://github.com/davidparks11/resume").spawn() {
                    eprintln!("Failed to open URL: {}", e);
                }
            }
            _ => {}
        }
    }
    

    fn handle_menu_keys(&mut self, key: event::KeyEvent) {
        match key.code {
            KeyCode::Char('q') | KeyCode::Esc => {
                self.should_exit = true;
            }
            KeyCode::Up | KeyCode::Char('k') => {
                if self.menu_index > 0 {
                    self.menu_index -= 1;
                    self.switch_to_selected_screen();
                }
            }
            KeyCode::Down | KeyCode::Char('j') => {
                if self.menu_index < 5 {
                    self.menu_index += 1;
                    self.switch_to_selected_screen();
                }
            }
            KeyCode::Enter => {
                self.switch_to_selected_screen();
            }
            _ => {}
        }
    }

    fn handle_content_keys(&mut self, key: event::KeyEvent) {
        match key.code {
            KeyCode::Char('q') => {
                self.should_exit = true;
            }
            KeyCode::Esc | KeyCode::Backspace => {
                self.previous_mode = self.display_mode;
                self.display_mode = DisplayMode::Menu;
            }
            KeyCode::Up | KeyCode::Char('k') => {
                if self.menu_index > 0 {
                    self.menu_index -= 1;
                    self.switch_to_selected_screen();
                }
            }
            KeyCode::Down | KeyCode::Char('j') => {
                if self.menu_index < 5 {
                    self.menu_index += 1;
                    self.switch_to_selected_screen();
                }
            }
            KeyCode::Right | KeyCode::Char('l') => {
                if self.display_mode == DisplayMode::Projects {
                    self.previous_mode = self.display_mode;
                    self.display_mode = DisplayMode::ProjectLinks;
                    self.link_index = 0;
                }
                // Skills text mode should no longer be accessible directly, 
                // but leaving handler in case of future changes
            }
            KeyCode::Enter => {
                self.switch_to_selected_screen();
            }
            _ => {}
        }
    }
}