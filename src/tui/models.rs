use serde::{Deserialize, Serialize};

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
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
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
    pub year: u16,
    #[serde(rename = "type")]
    pub event_type: TimelineType,
    pub title: String,
    pub organization: String,
    pub description: String,
    pub highlights: Option<Vec<String>>,
    pub technologies: Option<Vec<String>>,
}

/// Timeline filter type - kept as a placeholder but now only has one option
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum TimelineFilter {
    All,
}

/// Complete timeline data
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TimelineData {
    pub timeline: Vec<TimelineEvent>,
}

/// Hyperlink information
#[derive(Debug, Clone)]
pub struct Link {
    pub text: String,
    pub url: String,
    pub line: usize,
    pub start_column: usize,
    pub end_column: usize,
}

/// Display modes for the application
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum DisplayMode {
    Menu,
    About,
    Skills,
    SkillsVisual,
    Projects,
    ProjectLinks,
    Timeline,
    Contact,
}