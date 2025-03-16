use ratatui::{
    layout::{Constraint, Direction, Layout, Alignment, Margin},
    style::{Color, Modifier, Style},
    text::{Line, Span},
    widgets::{Block, Borders, List, ListItem, Paragraph, Wrap, Gauge},
    Frame,
};

use super::app::{App, DisplayMode};
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
    }
}

/// Renders the menu sidebar (always visible)
fn render_menu_sidebar(f: &mut Frame, app: &mut App, area: ratatui::layout::Rect) {
    let menu_items = vec![
        "About Me",
        "Skills",
        "Projects",
        "Why Warp?",
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