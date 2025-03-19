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
                self.display_mode = DisplayMode::Skills;
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
                    self.previous_mode = DisplayMode::Timeline;
                    self.menu_index = 4;
                    self.display_mode = DisplayMode::Menu;
                    self.timeline_detail_view = false;
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
                self.display_mode = DisplayMode::Skills;
                self.skill_category_index = 0;
            }
            KeyCode::Left | KeyCode::Char('h') => {
                if self.skill_category_index == 0 {
                    self.display_mode = DisplayMode::Skills;
                } else {
                    self.skill_category_index -= 1;
                }
            }
            KeyCode::Right | KeyCode::Char('l') => {
                if !self.skills_data.categories.is_empty() && 
                   self.skill_category_index < self.skills_data.categories.len() - 1 {
                    self.skill_category_index += 1;
                }
            }
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
                else if self.display_mode == DisplayMode::Skills {
                    self.previous_mode = self.display_mode;
                    self.display_mode = DisplayMode::SkillsVisual;
                    self.skill_category_index = 0;
                }
            }
            KeyCode::Enter => {
                self.switch_to_selected_screen();
            }
            _ => {}
        }
    }
}