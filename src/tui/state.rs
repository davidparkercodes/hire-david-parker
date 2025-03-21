use super::models::{DisplayMode, SkillsData, TimelineEvent, TimelineFilter, TimelineType};
use crate::{about, skills, projects, welcome, timeline, contact, load_timeline_data, load_project_links, ProjectLink};
use std::error::Error;
use std::path::Path;

pub struct App {
    pub menu_index: usize,
    pub link_index: usize,
    pub skill_category_index: usize,
    pub display_mode: DisplayMode,
    pub previous_mode: DisplayMode,
    pub timeline_filter: TimelineFilter,
    pub timeline_event_index: usize,
    pub timeline_detail_view: bool,
    pub about_content: String,
    pub skills_content: String,
    pub skills_data: SkillsData,
    pub skills_page: usize,
    pub projects_content: String,
    pub project_links: Vec<ProjectLink>,
    pub welcome_content: String,
    pub timeline_content: String,
    pub contact_content: String,
    pub timeline_events: Vec<TimelineEvent>,
    pub timeline_index: usize,
    pub should_exit: bool,
    pub skip_auto_switch: bool,
}

impl App {
    pub fn new() -> Self {
        let mut timeline_events = load_timeline_data().unwrap_or_default();
        
        timeline_events.sort_by_key(|event| event.year);
        
        let timeline_index = 0;
        
        let project_links = load_project_links()
            .map(|pl| pl.links)
            .unwrap_or_default();
        
        Self {
            menu_index: 0,
            link_index: 0,
            skill_category_index: 0,
            display_mode: DisplayMode::About,
            previous_mode: DisplayMode::Menu,
            timeline_filter: TimelineFilter::All,
            timeline_event_index: 0,
            timeline_detail_view: false,
            about_content: about(),
            skills_content: skills(),
            skills_data: load_skills_data().unwrap_or_default(),
            skills_page: 0,
            projects_content: projects(),
            project_links,
            welcome_content: welcome(),
            timeline_content: timeline(),
            contact_content: contact(),
            timeline_events: timeline_events.into_iter().map(|e| TimelineEvent {
                year: e.year as u16,
                event_type: match e.year % 5 {
                    0 => TimelineType::Career,
                    1 => TimelineType::Education,
                    2 => TimelineType::Certification,
                    3 => TimelineType::Project,
                    _ => TimelineType::Other,
                },
                title: e.title,
                organization: e.organization,
                description: e.description,
                highlights: Some(e.highlights),
                technologies: Some(e.technologies),
            }).collect(),
            timeline_index,
            should_exit: false,
            skip_auto_switch: false,
        }
    }
    
    pub fn get_filtered_events(&self) -> Vec<&TimelineEvent> {
        self.timeline_events.iter().collect()
    }
}

fn load_skills_data() -> Result<SkillsData, Box<dyn Error>> {
    let path = Path::new(env!("CARGO_MANIFEST_DIR"))
        .join("src")
        .join("static")
        .join("content")
        .join("skills.json");
    
    let content = std::fs::read_to_string(&path)?;
    let skills_data: SkillsData = serde_json::from_str(&content)?;
    Ok(skills_data)
}
