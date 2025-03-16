use ratatui::{
    layout::{Constraint, Direction, Layout, Alignment, Margin, Rect},
    style::{Color, Modifier, Style},
    text::{Line, Span},
    widgets::{Block, Borders, List, ListItem, Paragraph, Wrap, Gauge},
    Frame,
};

use super::app::{App, DisplayMode, TimelineCategory, TimelineEvent};
use super::markdown::parse_markdown;

/// Renders the user interface widgets
pub fn render(f: &mut Frame, app: &mut App) {
    // Create the layout
    let chunks = Layout::default()
        .direction(Direction::Vertical)
        .constraints(
            [
                Constraint::Length(3),  // Title
                Constraint::Min(0),     // Content
                Constraint::Length(3),  // Footer
            ]
            .as_ref(),
        )
        .split(f.area());

    // Create title
    let title = Paragraph::new("David Parker - Interactive Resume")
        .style(Style::default().add_modifier(Modifier::BOLD))
        .alignment(Alignment::Center)
        .block(Block::default().borders(Borders::ALL).border_style(Style::default().fg(Color::Cyan)));
    f.render_widget(title, chunks[0]);

    // Create footer
    let footer_text = match app.display_mode {
        DisplayMode::Menu => "q: Quit | ↑/k: Up | ↓/j: Down | Enter: Select",
        DisplayMode::Projects => "q: Quit | ↑/k: Up | ↓/j: Down | →/l: Navigate Links | Esc: Return to Menu",
        DisplayMode::ProjectLinks => "q: Quit | ↑/k: Up | ↓/j: Down | Enter: Open Link | ←/h: Back to Projects",
        DisplayMode::Skills => "q: Quit | ↑/k: Up | ↓/j: Down | →/l: View Skill Meters | Esc: Return to Menu",
        DisplayMode::SkillsVisual => "q: Quit | ←/h: Previous Category | →/l: Next Category | Esc: Back to Skills",
        DisplayMode::Contact => "q: Quit | Esc: Return to Menu",
        DisplayMode::Timeline => "q: Quit | Tab: Switch Category | ←/h: Previous | →/l: Next | Enter: View Details | Esc: Menu",
        DisplayMode::TimelineDetail => "q: Quit | ←/h: Previous Entry | →/l: Next Entry | Tab: Switch Category | Esc: Back to Timeline",
        _ => "q: Quit | ↑/k: Up | ↓/j: Down | Enter: Select | Esc: Return to Menu",
    };
    let footer = Paragraph::new(footer_text)
        .alignment(Alignment::Center)
        .block(Block::default().borders(Borders::ALL).border_style(Style::default().fg(Color::DarkGray)));
    f.render_widget(footer, chunks[2]);

    // Create the main content area layout with menu always visible
    let content_chunks = Layout::default()
        .direction(Direction::Horizontal)
        .constraints([Constraint::Percentage(25), Constraint::Percentage(75)].as_ref())
        .split(chunks[1]);
    
    // Always render the menu on the left side
    render_menu_sidebar(f, app, content_chunks[0]);
    
    // Render the appropriate content on the right side
    match app.display_mode {
        DisplayMode::Menu => render_welcome(f, app, content_chunks[1]),
        DisplayMode::About => render_about(f, app, content_chunks[1]),
        DisplayMode::Skills => render_skills(f, app, content_chunks[1]),
        DisplayMode::SkillsVisual => render_skills_visual(f, app, content_chunks[1]),
        DisplayMode::Projects => render_projects(f, app, content_chunks[1]),
        DisplayMode::ProjectLinks => render_project_links(f, app, content_chunks[1]),
        DisplayMode::WhyWarp => render_why_warp(f, app, content_chunks[1]),
        DisplayMode::Contact => render_contact(f, app, content_chunks[1]),
        DisplayMode::Timeline => render_timeline(f, app, content_chunks[1]),
        DisplayMode::TimelineDetail => render_timeline_detail(f, app, content_chunks[1]),
    }
}

/// Renders the menu sidebar (always visible)
fn render_menu_sidebar(f: &mut Frame, app: &mut App, area: ratatui::layout::Rect) {
    let menu_items = vec![
        "About Me",
        "Skills",
        "Projects",
        "Why Warp?",
        "Contact",
        "Timeline",
    ];

    let items: Vec<ListItem> = menu_items
        .iter()
        .enumerate()
        .map(|(i, item)| {
            let style = if i == app.menu_index {
                // Keep highlight color for selected item only
                Style::default().fg(Color::Yellow).add_modifier(Modifier::BOLD)
            } else {
                Style::default()
            };
            ListItem::new(Line::from(Span::styled(*item, style)))
        })
        .collect();

    let menu = List::new(items)
        .block(Block::default().title("Menu").borders(Borders::ALL).border_style(Style::default().fg(Color::Blue)))
        .highlight_style(Style::default().add_modifier(Modifier::BOLD));

    f.render_widget(menu, area);
}
/// Renders the welcome screen
fn render_welcome(f: &mut Frame, app: &mut App, area: ratatui::layout::Rect) {
    let (text, links) = parse_markdown(&app.welcome_content);
    let instructions = Paragraph::new(text)
        .block(Block::default().title("Instructions").borders(Borders::ALL).border_style(Style::default().fg(Color::Blue)))
        .wrap(Wrap { trim: true });

    app.links = links;
    f.render_widget(instructions, area);
}

/// Renders the about section
fn render_about(f: &mut Frame, app: &mut App, area: ratatui::layout::Rect) {
    let (text, links) = parse_markdown(&app.about_content);
    let paragraph = Paragraph::new(text)
        .block(Block::default().title("About Me").borders(Borders::ALL).border_style(Style::default().fg(Color::Blue)))
        .wrap(Wrap { trim: true });
    
    app.links = links;
    f.render_widget(paragraph, area);
}

/// Renders the skills section
fn render_skills(f: &mut Frame, app: &mut App, area: ratatui::layout::Rect) {
    let (text, links) = parse_markdown(&app.skills_content);
    let paragraph = Paragraph::new(text)
        .block(Block::default().title("Skills (→ for bar graphs)").borders(Borders::ALL).border_style(Style::default().fg(Color::Blue)))
        .wrap(Wrap { trim: true });
    
    app.links = links;
    f.render_widget(paragraph, area);
}

/// Renders the projects section
fn render_projects(f: &mut Frame, app: &mut App, area: ratatui::layout::Rect) {
    let (text, links) = parse_markdown(&app.projects_content);
    
    let title = "Projects (→ for links)";
    
    let paragraph = Paragraph::new(text)
        .block(Block::default().title(title).borders(Borders::ALL).border_style(Style::default().fg(Color::Blue)))
        .wrap(Wrap { trim: true });
    
    app.links = links;
    f.render_widget(paragraph, area);
}

/// Renders the project links for navigation
fn render_project_links(f: &mut Frame, app: &mut App, area: ratatui::layout::Rect) {
    if app.links.is_empty() {
        return;
    }
    
    let items: Vec<ListItem> = app.links
        .iter()
        .enumerate()
        .map(|(i, link)| {
            let style = if i == app.link_index {
                Style::default().fg(Color::Yellow).add_modifier(Modifier::BOLD)
            } else {
                Style::default().fg(Color::Cyan).add_modifier(Modifier::UNDERLINED)
            };
            
            ListItem::new(Line::from(vec![
                Span::raw("  "),
                Span::styled(&link.text, style),
                Span::raw(" - "),
                Span::styled(&link.url, Style::default().fg(Color::DarkGray)),
            ]))
        })
        .collect();
    
    let links_list = List::new(items)
        .block(Block::default()
            .title("Project Links (Enter to open, ← to go back)")
            .borders(Borders::ALL)
            .border_style(Style::default().fg(Color::Blue)))
        .highlight_style(Style::default().add_modifier(Modifier::BOLD));
    
    f.render_widget(links_list, area);
}

/// Renders the "Why Warp?" section
fn render_why_warp(f: &mut Frame, app: &mut App, area: ratatui::layout::Rect) {
    let (text, links) = parse_markdown(&app.why_warp_content);
    let paragraph = Paragraph::new(text)
        .block(Block::default().title("Why Warp?").borders(Borders::ALL).border_style(Style::default().fg(Color::Blue)))
        .wrap(Wrap { trim: true });
    
    app.links = links;
    f.render_widget(paragraph, area);
}

/// Renders the Contact section
fn render_contact(f: &mut Frame, app: &mut App, area: ratatui::layout::Rect) {
    let (text, links) = parse_markdown(&app.contact_content);
    let paragraph = Paragraph::new(text)
        .block(Block::default().title("Contact").borders(Borders::ALL).border_style(Style::default().fg(Color::Blue)))
        .wrap(Wrap { trim: true });
    
    app.links = links;
    f.render_widget(paragraph, area);
}

/// Renders the skills visualization with interactive bar meters
fn render_skills_visual(f: &mut Frame, app: &App, area: ratatui::layout::Rect) {
    // Create the block that will contain all the skills
    let block = Block::default()
        .title("Skill Proficiency (← → to navigate categories)")
        .borders(Borders::ALL)
        .border_style(Style::default().fg(Color::Blue));
    
    f.render_widget(block.clone(), area);
    
    // If there's no skill data, show a message
    if app.skills_data.categories.is_empty() {
        let message = Paragraph::new("No skill data available")
            .alignment(Alignment::Center)
            .wrap(Wrap { trim: true });
        let inner_area = area.inner(Margin { vertical: 1, horizontal: 2 });
        f.render_widget(message, inner_area);
        return;
    }
    
    // Show category selection at the top
    let category_chunks = Layout::default()
        .direction(Direction::Vertical)
        .constraints([
            Constraint::Length(3), // Category title
            Constraint::Min(0),    // Skills
        ])
        .margin(1)
        .split(area);
    
    // Create the category tabs
    let category_names: Vec<&str> = app.skills_data.categories
        .iter()
        .map(|c| c.name.as_str())
        .collect();
    
    let category_spans: Vec<Span> = category_names
        .iter()
        .enumerate()
        .map(|(i, &name)| {
            if i == app.skill_category_index {
                // Add left/right arrows to indicate navigation direction
                Span::styled(
                    format!("《 {} 》", name),
                    Style::default().fg(Color::Yellow).add_modifier(Modifier::BOLD),
                )
            } else {
                Span::styled(
                    format!(" {} ", name),
                    Style::default().fg(Color::White),
                )
            }
        })
        .collect();
    
    let mut category_line = Line::from(vec![]);
    for (i, span) in category_spans.into_iter().enumerate() {
        category_line.spans.push(span);
        if i < category_names.len() - 1 {
            category_line.spans.push(Span::raw(" | "));
        }
    }
    
    let category_selector = Paragraph::new(category_line)
        .alignment(Alignment::Center);
    
    f.render_widget(category_selector, category_chunks[0]);
    
    // If we have a valid selected category, show its skills
    if app.skill_category_index < app.skills_data.categories.len() {
        let current_category = &app.skills_data.categories[app.skill_category_index];
        let skills = &current_category.skills;
        
        if !skills.is_empty() {
            // Determine how many skills we have and create constraints for each
            let skills_constraints = skills.iter()
                .map(|_| Constraint::Length(3))
                .collect::<Vec<_>>();
            
            let skills_layout = Layout::default()
                .direction(Direction::Vertical)
                .constraints(skills_constraints)
                .spacing(1)
                .split(category_chunks[1]);
            
            // Render each skill bar
            for (i, skill) in skills.iter().enumerate() {
                if i < skills_layout.len() {
                    let skill_area = skills_layout[i];
                    
                    // Determine color based on skill level
                    let bar_color = match skill.level {
                        0..=25 => Color::Red,
                        26..=50 => Color::Yellow,
                        51..=75 => Color::Green,
                        _ => Color::Cyan,
                    };
                    
                    let gauge = Gauge::default()
                        .block(Block::default().title(Span::styled(
                            &skill.name,
                            Style::default().add_modifier(Modifier::BOLD),
                        )))
                        .gauge_style(Style::default().fg(bar_color).bg(Color::DarkGray))
                        .percent(skill.level as u16)
                        .label(Span::styled(
                            format!("{}%", skill.level),
                            Style::default().fg(Color::White).add_modifier(Modifier::BOLD),
                        ));
                    
                    f.render_widget(gauge, skill_area);
                }
            }
        }
    }
}

// We're now using ratatui's built-in Margin

/// Renders the timeline section
fn render_timeline(f: &mut Frame, app: &mut App, area: ratatui::layout::Rect) {
    // First parse the markdown content for the timeline introduction
    let (text, links) = parse_markdown(&app.timeline_content);
    app.links = links;
    
    // Create the layout with introduction at top and timeline cards below
    let chunks = Layout::default()
        .direction(Direction::Vertical)
        .constraints([
            Constraint::Length(8),  // Intro section
            Constraint::Min(0),     // Timeline cards
        ])
        .split(area);
    
    // Render the introduction paragraph
    let intro = Paragraph::new(text)
        .block(Block::default().borders(Borders::ALL).border_style(Style::default().fg(Color::Blue)))
        .wrap(Wrap { trim: true });
    f.render_widget(intro, chunks[0]);
    
    // Create title based on current category
    let title = match app.timeline_category {
        TimelineCategory::Career => "Career History",
        TimelineCategory::Education => "Education",
        TimelineCategory::Certifications => "Certifications",
    };
    
    let events = match app.timeline_category {
        TimelineCategory::Career => &app.timeline_data.events,
        TimelineCategory::Education => &app.timeline_data.education,
        TimelineCategory::Certifications => &app.timeline_data.certifications,
    };
    
    if events.is_empty() {
        // Show message if no events are available
        let message = Paragraph::new("No timeline data available for this category")
            .alignment(Alignment::Center)
            .block(Block::default().title(title).borders(Borders::ALL).border_style(Style::default().fg(Color::Blue)));
        f.render_widget(message, chunks[1]);
        return;
    }
    
    // If we have a valid event index
    if app.timeline_event_index < events.len() {
        let event = &events[app.timeline_event_index];
        
        // Render the timeline event card
        render_timeline_card(f, app, chunks[1], event, title);
    } else {
        // Reset the index if it's out of bounds
        app.timeline_event_index = 0;
        if !events.is_empty() {
            let event = &events[0];
            render_timeline_card(f, app, chunks[1], event, title);
        }
    }
}

/// Renders a single timeline event card
fn render_timeline_card(f: &mut Frame, app: &App, area: ratatui::layout::Rect, event: &TimelineEvent, _title: &str) {
    // Create card layout with margins (kept for future use)
    let _inner_area = area.inner(Margin { vertical: 1, horizontal: 2 });
    
    let card_title = match app.timeline_category {
        TimelineCategory::Career => {
            format!("{} - {} at {}", 
                    event.year, 
                    event.title, 
                    event.company.as_deref().unwrap_or("Unknown"))
        },
        TimelineCategory::Education => {
            format!("{} - {} from {}", 
                    event.year, 
                    event.degree.as_deref().unwrap_or("Degree"), 
                    event.institution.as_deref().unwrap_or("Unknown"))
        },
        TimelineCategory::Certifications => {
            format!("{} - {} by {}", 
                    event.year, 
                    event.title, 
                    event.organization.as_deref().unwrap_or("Unknown"))
        },
    };
    
    // Create navigation indicator showing position (e.g., "2 of 5")
    let events = match app.timeline_category {
        TimelineCategory::Career => &app.timeline_data.events,
        TimelineCategory::Education => &app.timeline_data.education,
        TimelineCategory::Certifications => &app.timeline_data.certifications,
    };
    
    // Add navigation arrows to indicate previous/next entry availability
    let mut nav_arrows = String::new();
    if app.timeline_event_index > 0 {
        nav_arrows.push_str("◀ ");
    } else {
        nav_arrows.push_str("  ");
    }
    
    nav_arrows.push_str(&format!("{} of {}", app.timeline_event_index + 1, events.len()));
    
    if app.timeline_event_index < events.len() - 1 {
        nav_arrows.push_str(" ▶");
    }
    
    let position_text = format!("{} (Tab to switch category, Enter for details)", nav_arrows);
    
    // Create card content
    let content = vec![
        Line::from(vec![
            Span::styled("Year: ", Style::default().add_modifier(Modifier::BOLD)),
            Span::raw(event.year.to_string()),
        ]),
        Line::from(Span::raw("")), // Empty line
        Line::from(vec![
            Span::styled("Title: ", Style::default().add_modifier(Modifier::BOLD)),
            Span::raw(&event.title),
        ]),
        Line::from(Span::raw("")), // Empty line
        Line::from(vec![
            Span::styled("Description: ", Style::default().add_modifier(Modifier::BOLD)),
            Span::raw(&event.description),
        ]),
    ];
    
    // Create a block with added visual cues for navigation in the title
    let block = Block::default()
        .title(Span::styled(card_title, Style::default().add_modifier(Modifier::BOLD)))
        .title_alignment(Alignment::Center)
        .borders(Borders::ALL)
        .border_style(Style::default().fg(Color::Blue));
    
    let card = Paragraph::new(content)
        .block(block)
        .alignment(Alignment::Left)
        .wrap(Wrap { trim: true });
    
    f.render_widget(card, area);
    
    // Add navigation indicator at the bottom
    let nav_area = Rect::new(
        area.x,
        area.y + area.height - 2,
        area.width,
        1,
    );
    
    let navigation = Paragraph::new(position_text)
        .alignment(Alignment::Center)
        .style(Style::default().fg(Color::DarkGray));
    
    f.render_widget(navigation, nav_area);
}

/// Renders detailed view of a timeline event
fn render_timeline_detail(f: &mut Frame, app: &App, area: ratatui::layout::Rect) {
    let events = match app.timeline_category {
        TimelineCategory::Career => &app.timeline_data.events,
        TimelineCategory::Education => &app.timeline_data.education,
        TimelineCategory::Certifications => &app.timeline_data.certifications,
    };
    
    if events.is_empty() || app.timeline_event_index >= events.len() {
        return;
    }
    
    let event = &events[app.timeline_event_index];
    
    // Title based on event is created but not directly used (kept for future use)
    let _title = match app.timeline_category {
        TimelineCategory::Career => {
            format!("{} - {} at {}", 
                    event.year, 
                    event.title, 
                    event.company.as_deref().unwrap_or("Unknown"))
        },
        TimelineCategory::Education => {
            format!("{} - {} from {}", 
                    event.year, 
                    event.degree.as_deref().unwrap_or("Degree"), 
                    event.institution.as_deref().unwrap_or("Unknown"))
        },
        TimelineCategory::Certifications => {
            format!("{} - {} by {}", 
                    event.year, 
                    event.title, 
                    event.organization.as_deref().unwrap_or("Unknown"))
        },
    };
    
    // Create layout with sections for different parts of the detail view
    let chunks = Layout::default()
        .direction(Direction::Vertical)
        .constraints([
            Constraint::Length(3),  // Header with navigation
            Constraint::Length(3),  // Year and title
            Constraint::Length(6),  // Description
            Constraint::Min(0),     // Highlights/technologies
        ])
        .margin(1)
        .split(area);
    
    // Render header with category and navigation
    let category_name = match app.timeline_category {
        TimelineCategory::Career => "Career History",
        TimelineCategory::Education => "Education",
        TimelineCategory::Certifications => "Certifications",
    };
    
    // Add navigation info to the header
    let navigation_text = format!("{} ({} of {}) [Esc to go back]", 
                                 category_name,
                                 app.timeline_event_index + 1,
                                 events.len());
    
    let header = Paragraph::new(navigation_text)
        .alignment(Alignment::Center)
        .style(Style::default().fg(Color::Yellow).add_modifier(Modifier::BOLD))
        .block(Block::default().borders(Borders::ALL).border_style(Style::default().fg(Color::Blue)));
    
    f.render_widget(header, chunks[0]);
    
    // Render title section with enhanced style
    let place = match app.timeline_category {
        TimelineCategory::Career => event.company.as_deref().unwrap_or("Unknown").to_string(),
        TimelineCategory::Education => event.institution.as_deref().unwrap_or("Unknown").to_string(),
        TimelineCategory::Certifications => event.organization.as_deref().unwrap_or("Unknown").to_string(),
    };
    
    let title_content = vec![
        Line::from(vec![
            Span::styled("Year: ", Style::default().fg(Color::Cyan).add_modifier(Modifier::BOLD)),
            Span::raw(event.year.to_string()),
        ]),
        Line::from(vec![
            Span::styled("Title: ", Style::default().fg(Color::Cyan).add_modifier(Modifier::BOLD)),
            Span::raw(&event.title),
        ]),
        Line::from(vec![
            Span::styled(match app.timeline_category {
                TimelineCategory::Career => "Company: ",
                TimelineCategory::Education => "Institution: ",
                TimelineCategory::Certifications => "Organization: ",
            }, Style::default().fg(Color::Cyan).add_modifier(Modifier::BOLD)),
            Span::raw(&place),
        ]),
    ];
    
    let title_widget = Paragraph::new(title_content)
        .block(Block::default().borders(Borders::ALL).border_style(Style::default().fg(Color::Blue)))
        .wrap(Wrap { trim: true });
    
    f.render_widget(title_widget, chunks[1]);
    
    // Render description with scrollable option
    let desc_widget = Paragraph::new(event.description.clone())
        .block(Block::default()
            .title("Description")
            .borders(Borders::ALL)
            .border_style(Style::default().fg(Color::Blue)))
        .wrap(Wrap { trim: true })
        .style(Style::default());
    
    f.render_widget(desc_widget, chunks[2]);
    
    // Render highlights and technologies (if available)
    let highlights_chunk = Layout::default()
        .direction(Direction::Horizontal)
        .constraints([
            Constraint::Percentage(60),
            Constraint::Percentage(40),
        ])
        .split(chunks[3]);
    
    // Render highlights with colored bullets if available
    let highlights_content = match &event.highlights {
        Some(highlights) if !highlights.is_empty() => {
            let items: Vec<ListItem> = highlights
                .iter()
                .map(|highlight| {
                    ListItem::new(Line::from(vec![
                        Span::styled(" ► ", Style::default().fg(Color::Yellow)),
                        Span::raw(highlight),
                    ]))
                })
                .collect();
            
            List::new(items)
                .block(Block::default()
                    .title(Span::styled("Key Highlights", Style::default().add_modifier(Modifier::BOLD)))
                    .borders(Borders::ALL)
                    .border_style(Style::default().fg(Color::Blue)))
                .highlight_style(Style::default().add_modifier(Modifier::BOLD))
        },
        _ => {
            List::new(vec![ListItem::new("No highlights available")])
                .block(Block::default().title("Highlights").borders(Borders::ALL).border_style(Style::default().fg(Color::Blue)))
        }
    };
    
    f.render_widget(highlights_content, highlights_chunk[0]);
    
    // Render technologies with visual enhancement if available
    let tech_content = if let Some(technologies) = &event.technologies {
        if !technologies.is_empty() {
            // Create a list of technologies with styled badges
            let items: Vec<ListItem> = technologies
                .iter()
                .map(|tech| {
                    ListItem::new(Line::from(vec![
                        Span::styled(format!(" [{}] ", tech), 
                            Style::default()
                                .fg(Color::Black)
                                .bg(Color::Cyan)
                                .add_modifier(Modifier::BOLD)),
                    ]))
                })
                .collect();
            
            List::new(items)
                .block(Block::default()
                    .title(Span::styled("Technologies", Style::default().add_modifier(Modifier::BOLD)))
                    .borders(Borders::ALL)
                    .border_style(Style::default().fg(Color::Blue)))
                .style(Style::default())
        } else {
            List::new(vec![ListItem::new("No technology information available")])
                .block(Block::default().title("Technologies").borders(Borders::ALL).border_style(Style::default().fg(Color::Blue)))
        }
    } else {
        List::new(vec![ListItem::new("No technology information available")])
            .block(Block::default().title("Technologies").borders(Borders::ALL).border_style(Style::default().fg(Color::Blue)))
    };
    
    f.render_widget(tech_content, highlights_chunk[1]);
}