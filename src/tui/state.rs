use super::models::{DisplayMode, SkillsData, TimelineEvent, TimelineFilter, TimelineType};
use crate::{about, skills, projects, why_warp, welcome, timeline, load_timeline_data};
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
    pub projects_content: String,
    pub why_warp_content: String,
    pub welcome_content: String,
    pub timeline_content: String,
    pub timeline_events: Vec<TimelineEvent>,
    pub timeline_index: usize,
    pub should_exit: bool,
}

impl App {
    pub fn new() -> Self {
        let mut timeline_events = load_timeline_data().unwrap_or_default();
        
        timeline_events.sort_by_key(|event| event.year);
        
        let timeline_index = 0;
        
        Self {
            menu_index: 0,
            link_index: 0,
            skill_category_index: 0,
            display_mode: DisplayMode::Menu,
            previous_mode: DisplayMode::Menu,
            timeline_filter: TimelineFilter::All,
            timeline_event_index: 0,
            timeline_detail_view: false,
            about_content: about(),
            skills_content: skills(),
            skills_data: load_skills_data().unwrap_or_default(),
            projects_content: projects(),
            why_warp_content: why_warp(),
            welcome_content: welcome(),
            timeline_content: timeline(),
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