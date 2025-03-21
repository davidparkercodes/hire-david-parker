use ratatui::{
    layout::{Constraint, Direction, Layout, Alignment, Rect},
    style::{Color, Modifier, Style},
    text::{Line, Span},
    widgets::{Block, Borders, List, ListItem, Paragraph, Wrap, Gauge},
    Frame,
};

use super::state::App;
use super::models::DisplayMode;
use super::markdown::parse_markdown;

/// Renders the user interface widgets
pub fn render(f: &mut Frame, app: &mut App) {
    let chunks = Layout::default()
        .direction(Direction::Vertical)
        .constraints(
            [
                Constraint::Length(3),
                Constraint::Min(0),
                Constraint::Length(3),
            ]
            .as_ref(),
        )
        .split(f.area());

    let title = Paragraph::new("David Parker - Interactive Resume")
        .style(Style::default().add_modifier(Modifier::BOLD))
        .alignment(Alignment::Center)
        .block(Block::default().borders(Borders::ALL).border_style(Style::default().fg(Color::Cyan)));
    f.render_widget(title, chunks[0]);

    let footer_text = match app.display_mode {
        DisplayMode::Menu => "q: Quit | ↑/k: Up | ↓/j: Down | Enter: Select",
        DisplayMode::Timeline => "q: Quit | ←/h: Previous | →/l: Next | Esc: Return to Menu",
        DisplayMode::SkillsVisual => {
            let has_multiple_pages = if !app.skills_data.categories.is_empty() {
                let category_index = app.skill_category_index.min(app.skills_data.categories.len() - 1);
                let category = &app.skills_data.categories[category_index];
                
                let skills_per_page = 5;
                let total_skills = category.skills.len();
                
                total_skills > skills_per_page
            } else {
                false
            };
            
            if has_multiple_pages {
                "q: Quit | ↑/↓: Categories | ←/→: Pages | Esc: Return to Menu"
            } else {
                "q: Quit | ↑/↓: Categories | Esc: Return to Menu"
            }
        },
        _ => "q: Quit | ↑/k: Up | ↓/j: Down | Enter: Select | Esc: Return to Menu",
    };
    let footer = Paragraph::new(footer_text)
        .alignment(Alignment::Center)
        .block(Block::default().borders(Borders::ALL).border_style(Style::default().fg(Color::DarkGray)));
    f.render_widget(footer, chunks[2]);

    let content_chunks = Layout::default()
        .direction(Direction::Horizontal)
        .constraints([Constraint::Percentage(25), Constraint::Percentage(75)].as_ref())
        .split(chunks[1]);
    
    render_menu_sidebar(f, app, content_chunks[0]);
    
    match app.display_mode {
        DisplayMode::Menu => {
            render_about(f, app, content_chunks[1])
        },
        DisplayMode::About => render_about(f, app, content_chunks[1]),
        DisplayMode::Skills => render_skills(f, app, content_chunks[1]),
        DisplayMode::SkillsVisual => render_skills_visual(f, app, content_chunks[1]),
        DisplayMode::Projects => render_projects(f, app, content_chunks[1]),
        DisplayMode::ProjectLinks => render_project_links(f, app, content_chunks[1]),
        DisplayMode::Timeline => render_timeline(f, app, content_chunks[1]),
        DisplayMode::Contact => render_contact(f, app, content_chunks[1]),
    }
}

/// Renders the menu sidebar (always visible)
fn render_menu_sidebar(f: &mut Frame, app: &mut App, area: ratatui::layout::Rect) {
    let menu_items = vec![
        "About Me",
        "Skills",
        "Projects",
        "Timeline",
        "Contact",
    ];

    let items: Vec<ListItem> = menu_items
        .iter()
        .enumerate()
        .map(|(i, item)| {
            let style = if i == app.menu_index {
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

/// Renders the about section
fn render_about(f: &mut Frame, app: &mut App, area: ratatui::layout::Rect) {
    let (mut text, _links) = parse_markdown(&app.about_content);
    
    for line in &mut text.lines {
        let line_content = line.spans.iter()
            .map(|span| span.content.to_string())
            .collect::<String>();
        
        if line_content.starts_with("->") && line_content.ends_with("<-") {
            *line = line.clone().alignment(ratatui::layout::Alignment::Center);
            
            if !line.spans.is_empty() {
                if let Some(first_span) = line.spans.first_mut() {
                    if first_span.content.starts_with("->") {
                        let new_content = first_span.content.to_string();
                        first_span.content = new_content[2..].to_string().into();
                    }
                }
                
                if let Some(last_span) = line.spans.last_mut() {
                    if last_span.content.ends_with("<-") {
                        let new_content = last_span.content.to_string();
                        last_span.content = new_content[..new_content.len()-2].to_string().into();
                    }
                }
            }
        }
    }
    
    let paragraph = Paragraph::new(text)
        .block(Block::default().title("About Me").borders(Borders::ALL).border_style(Style::default().fg(Color::Blue)))
        .wrap(Wrap { trim: true });
    
    f.render_widget(paragraph, area);
}

/// Renders the skills section
fn render_skills(f: &mut Frame, app: &mut App, area: ratatui::layout::Rect) {
    let (mut text, _links) = parse_markdown(&app.skills_content);
    
    for line in &mut text.lines {
        let line_content = line.spans.iter()
            .map(|span| span.content.to_string())
            .collect::<String>();
        
        if line_content.starts_with("->") && line_content.ends_with("<-") {
            *line = line.clone().alignment(ratatui::layout::Alignment::Center);
            
            if !line.spans.is_empty() {
                if let Some(first_span) = line.spans.first_mut() {
                    if first_span.content.starts_with("->") {
                        let new_content = first_span.content.to_string();
                        first_span.content = new_content[2..].to_string().into();
                    }
                }
                
                if let Some(last_span) = line.spans.last_mut() {
                    if last_span.content.ends_with("<-") {
                        let new_content = last_span.content.to_string();
                        last_span.content = new_content[..new_content.len()-2].to_string().into();
                    }
                }
            }
        }
    }
    
    let paragraph = Paragraph::new(text)
        .block(Block::default().title("Skills (→ for bar graphs)").borders(Borders::ALL).border_style(Style::default().fg(Color::Blue)))
        .wrap(Wrap { trim: true });
    
    f.render_widget(paragraph, area);
}

/// Renders the projects section
fn render_projects(f: &mut Frame, app: &mut App, area: ratatui::layout::Rect) {
    let (mut text, _links) = parse_markdown(&app.projects_content);
    
    for line in &mut text.lines {
        let line_content = line.spans.iter()
            .map(|span| span.content.to_string())
            .collect::<String>();
        
        if line_content.starts_with("->") && line_content.ends_with("<-") {
            *line = line.clone().alignment(ratatui::layout::Alignment::Center);
            
            if !line.spans.is_empty() {
                if let Some(first_span) = line.spans.first_mut() {
                    if first_span.content.starts_with("->") {
                        let new_content = first_span.content.to_string();
                        first_span.content = new_content[2..].to_string().into();
                    }
                }
                
                if let Some(last_span) = line.spans.last_mut() {
                    if last_span.content.ends_with("<-") {
                        let new_content = last_span.content.to_string();
                        last_span.content = new_content[..new_content.len()-2].to_string().into();
                    }
                }
            }
        }
    }
    
    let title = "Projects (→ for links)";
    
    let paragraph = Paragraph::new(text)
        .block(Block::default().title(title).borders(Borders::ALL).border_style(Style::default().fg(Color::Blue)))
        .wrap(Wrap { trim: true });
    
    f.render_widget(paragraph, area);
}

/// Renders the project links for navigation
fn render_project_links(f: &mut Frame, app: &mut App, area: ratatui::layout::Rect) {
    if app.project_links.is_empty() {
        let message = Paragraph::new("No project links found.")
            .alignment(Alignment::Center)
            .block(Block::default()
                .title("Project Links")
                .borders(Borders::ALL)
                .border_style(Style::default().fg(Color::Blue)));
        
        f.render_widget(message, area);
        return;
    }
    
    let items: Vec<ListItem> = app.project_links
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

/// Renders the skills visualization with bar graphs
fn render_skills_visual(f: &mut Frame, app: &mut App, area: ratatui::layout::Rect) {
    if app.skills_data.categories.is_empty() {
        return;
    }
    
    let category_index = app.skill_category_index.min(app.skills_data.categories.len() - 1);
    let category = &app.skills_data.categories[category_index];
    
    let block = Block::default()
        .title(format!("Skills: {}", category.name))
        .borders(Borders::ALL)
        .border_style(Style::default().fg(Color::Blue));
    
    f.render_widget(block.clone(), area);
    
    let inner_area = block.inner(area);
    
    let chunks = Layout::default()
        .direction(Direction::Vertical)
        .constraints([
            Constraint::Length(2),
            Constraint::Min(4),
        ].as_ref())
        .split(inner_area);
    
    let mut header_text = vec![Span::styled(
        "Skills are shown in their original order.", 
        Style::default().fg(Color::Gray)
    )];
    
    let skills_area_height = chunks[1].height.saturating_sub(2);
    let skills_per_page = (skills_area_height / 3) as usize;
    
    let skills_per_page = skills_per_page.max(1);
    
    let total_pages = (category.skills.len() + skills_per_page - 1) / skills_per_page;
    
    if app.skills_page >= total_pages && total_pages > 0 {
        app.skills_page = total_pages - 1;
    }
    
    if total_pages > 1 {
        header_text.push(Span::raw(" "));
        header_text.push(Span::styled(
            format!("Page {}/{}. Use ←/→ to navigate pages.", app.skills_page + 1, total_pages),
            Style::default().fg(Color::Gray)
        ));
    }
    
    let header = Paragraph::new(Line::from(header_text))
        .alignment(Alignment::Center);
    f.render_widget(header, chunks[0]);
    
    let start_index = app.skills_page * skills_per_page;
    let end_index = (start_index + skills_per_page).min(category.skills.len());
    
    let skills_to_display = &category.skills[start_index..end_index];
    
    let mut bar_constraints = Vec::new();
    for _ in start_index..end_index {
        bar_constraints.push(Constraint::Length(1));
        bar_constraints.push(Constraint::Length(1));
        bar_constraints.push(Constraint::Length(1));
    }
    
    if bar_constraints.is_empty() {
        bar_constraints.push(Constraint::Min(1));
    }
    
    let skill_chunks = Layout::default()
        .direction(Direction::Vertical)
        .constraints(bar_constraints)
        .margin(1)
        .split(chunks[1]);
    
    
    for (display_idx, skill) in skills_to_display.iter().enumerate() {
        let chunk_base_idx = display_idx * 3;
        
        let name_paragraph = Paragraph::new(skill.name.as_str())
            .style(Style::default().fg(Color::Gray));
        
        if chunk_base_idx < skill_chunks.len() {
            f.render_widget(name_paragraph, skill_chunks[chunk_base_idx]);
        }
        
        if chunk_base_idx + 1 < skill_chunks.len() {
            let gauge = Gauge::default()
                .gauge_style(Style::default().fg(Color::Green))
                .ratio(skill.level as f64 / 100.0)
                .label(format!("{}%", skill.level));
            
            f.render_widget(gauge, skill_chunks[chunk_base_idx + 1]);
        }
        
    }
}


/// Renders the Contact Information section
fn render_contact(f: &mut Frame, app: &App, area: ratatui::layout::Rect) {
    let (mut text, _links) = parse_markdown(&app.contact_content);
    
    for line in &mut text.lines {
        let line_content = line.spans.iter()
            .map(|span| span.content.to_string())
            .collect::<String>();
        
        if line_content.starts_with("->") && line_content.ends_with("<-") {
            *line = line.clone().alignment(ratatui::layout::Alignment::Center);
            
            if !line.spans.is_empty() {
                if let Some(first_span) = line.spans.first_mut() {
                    if first_span.content.starts_with("->") {
                        let new_content = first_span.content.to_string();
                        first_span.content = new_content[2..].to_string().into();
                    }
                }
                
                if let Some(last_span) = line.spans.last_mut() {
                    if last_span.content.ends_with("<-") {
                        let new_content = last_span.content.to_string();
                        last_span.content = new_content[..new_content.len()-2].to_string().into();
                    }
                }
            }
        }
    }
    
    let paragraph = Paragraph::new(text)
        .block(Block::default().title("Contact Information").borders(Borders::ALL).border_style(Style::default().fg(Color::Blue)))
        .wrap(Wrap { trim: true });
    
    f.render_widget(paragraph, area);
}

/// Renders the Timeline section with a horizontal timeline visualization
fn render_timeline(f: &mut Frame, app: &App, area: ratatui::layout::Rect) {
    let chunks = Layout::default()
        .direction(Direction::Vertical)
        .constraints(
            [
                Constraint::Length(2),
                Constraint::Length(5),
                Constraint::Min(0),
            ]
            .as_ref(),
        )
        .split(area);
    
    let (mut text, _) = parse_markdown(&app.timeline_content);
    
    for line in &mut text.lines {
        let line_content = line.spans.iter()
            .map(|span| span.content.to_string())
            .collect::<String>();
        
        if line_content.starts_with("->") && line_content.ends_with("<-") {
            *line = line.clone().alignment(ratatui::layout::Alignment::Center);
            
            if !line.spans.is_empty() {
                if let Some(first_span) = line.spans.first_mut() {
                    if first_span.content.starts_with("->") {
                        let new_content = first_span.content.to_string();
                        first_span.content = new_content[2..].to_string().into();
                    }
                }
                
                if let Some(last_span) = line.spans.last_mut() {
                    if last_span.content.ends_with("<-") {
                        let new_content = last_span.content.to_string();
                        last_span.content = new_content[..new_content.len()-2].to_string().into();
                    }
                }
            }
        }
    }
    
    let instructions = Paragraph::new(text)
        .block(Block::default().title("Career Timeline").borders(Borders::ALL).border_style(Style::default().fg(Color::Blue)))
        .wrap(Wrap { trim: true });
    f.render_widget(instructions, chunks[0]);
    
    let timeline_area = chunks[1];
    
    if !app.timeline_events.is_empty() {
        let _ = app.timeline_index.min(app.timeline_events.len() - 1);
        
        render_horizontal_timeline(f, app, timeline_area);
        render_timeline_details(f, app, chunks[2]);
    } else {
        let empty_msg = Paragraph::new("No timeline events found.")
            .alignment(Alignment::Center)
            .block(Block::default().borders(Borders::ALL));
        f.render_widget(empty_msg, chunks[1]);
    }
}

/// Renders the horizontal timeline with year markers and points
fn render_horizontal_timeline(f: &mut Frame, app: &App, area: ratatui::layout::Rect) {
    let block = Block::default()
        .title("Navigate with ← →")
        .borders(Borders::ALL)
        .border_style(Style::default().fg(Color::Blue));
    f.render_widget(block.clone(), area);
    
    let inner_area = block.inner(area);
    
    let min_year = app.timeline_events.iter().map(|e| e.year).min().unwrap_or(2000);
    let max_year = app.timeline_events.iter().map(|e| e.year).max().unwrap_or(2024);
    
    let timeline_width = inner_area.width as usize;
    let year_span = (max_year - min_year) as usize;
    let pixels_per_year = if year_span > 0 {
        timeline_width / year_span
    } else {
        timeline_width
    };
    
    let horizontal_padding = 4;
    let usable_width = inner_area.width.saturating_sub(horizontal_padding * 2);
    
    let line_y = inner_area.y + inner_area.height / 2;
    
    let timeline_text = "─".repeat(usable_width as usize);
    let timeline_line = Line::from(Span::styled(
        timeline_text,
        Style::default().fg(Color::Gray)
    ));
    let timeline_paragraph = Paragraph::new(timeline_line);
    let timeline_area = Rect {
        x: inner_area.x + horizontal_padding,
        y: line_y,
        width: usable_width,
        height: 1,
    };
    f.render_widget(timeline_paragraph, timeline_area);
    
    let mut event_positions = Vec::new();
    
    for (i, event) in app.timeline_events.iter().enumerate() {
        let year_offset = (event.year - min_year) as usize;
        let width_ratio = usable_width as f32 / timeline_width as f32;
        let adjusted_offset = (year_offset as f32 * pixels_per_year as f32 * width_ratio) as u16;
        let x_pos = inner_area.x + horizontal_padding + adjusted_offset;
        
        event_positions.push(x_pos);
        
        if x_pos < inner_area.x + inner_area.width {
            let symbol = if i == app.timeline_index { "●" } else { "○" };
            let color = if i == app.timeline_index { Color::Yellow } else { Color::White };
            
            let point_paragraph = Paragraph::new(Line::from(Span::styled(
                symbol,
                Style::default().fg(color)
            )));
            
            let point_area = Rect {
                x: x_pos,
                y: line_y,
                width: 1,
                height: 1,
            };
            f.render_widget(point_paragraph, point_area);
            
            let year_text = event.year.to_string();
            let year_x = x_pos.saturating_sub((year_text.len() / 2) as u16);
            let year_len = year_text.len() as u16;
            
            if year_x + year_len < inner_area.x + inner_area.width {
                let year_paragraph = Paragraph::new(Line::from(Span::styled(
                    year_text,
                    Style::default().fg(color)
                )));
                
                let year_area = Rect {
                    x: year_x,
                    y: line_y + 1,
                    width: year_len,
                    height: 1,
                };
                f.render_widget(year_paragraph, year_area);
            }
        }
    }
}

/// Renders the details for the selected timeline event
fn render_timeline_details(f: &mut Frame, app: &App, area: ratatui::layout::Rect) {
    if app.timeline_events.is_empty() {
        return;
    }
    
    let event = &app.timeline_events[app.timeline_index];
    
    let chunks = Layout::default()
        .direction(Direction::Vertical)
        .constraints(
            [
                Constraint::Length(3),
                Constraint::Length(5),
                Constraint::Min(3),
                Constraint::Length(3),
            ]
            .as_ref(),
        )
        .split(area);
    
    let title = format!("{} | {}", event.title, event.organization);
    let title_paragraph = Paragraph::new(Line::from(vec![
        Span::styled(title, Style::default().fg(Color::Yellow).add_modifier(Modifier::BOLD))
    ]))
    .block(Block::default().borders(Borders::ALL).border_style(Style::default().fg(Color::Blue)))
    .alignment(Alignment::Center);
    f.render_widget(title_paragraph, chunks[0]);
    
    let desc_paragraph = Paragraph::new(Line::from(vec![
        Span::raw(event.description.clone())
    ]))
    .block(Block::default().title("Description").borders(Borders::ALL).border_style(Style::default().fg(Color::Blue)))
    .wrap(Wrap { trim: true });
    f.render_widget(desc_paragraph, chunks[1]);
    
    let highlights: Vec<ListItem> = event.highlights
        .as_ref()
        .map(|highlights| highlights.iter().map(|h| ListItem::new(Line::from(Span::raw(format!("• {}", h)))))
            .collect())
        .unwrap_or_default();
    
    let highlights_list = List::new(highlights)
        .block(Block::default().title("Highlights").borders(Borders::ALL).border_style(Style::default().fg(Color::Blue)))
        .style(Style::default());
    f.render_widget(highlights_list, chunks[2]);
    
    let tech_text = event.technologies.as_ref().map_or(String::new(), |techs| techs.join(" | "));
    let tech_paragraph = Paragraph::new(Line::from(vec![
        Span::styled(tech_text, Style::default().fg(Color::Green))
    ]))
    .block(Block::default().title("Technologies").borders(Borders::ALL).border_style(Style::default().fg(Color::Blue)))
    .alignment(Alignment::Center);
    f.render_widget(tech_paragraph, chunks[3]);
}
