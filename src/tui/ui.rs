use ratatui::{
    layout::{Constraint, Direction, Layout, Alignment},
    style::{Color, Modifier, Style},
    text::{Line, Span},
    widgets::{Block, Borders, List, ListItem, Paragraph, Wrap},
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
        .block(Block::default().title("Skills").borders(Borders::ALL).border_style(Style::default().fg(Color::Blue)))
        .wrap(Wrap { trim: true });
    
    app.links = links;
    f.render_widget(paragraph, area);
}

/// Renders the projects section
fn render_projects(f: &mut Frame, app: &mut App, area: ratatui::layout::Rect) {
    let (text, links) = parse_markdown(&app.projects_content);
    
    let title = if !links.is_empty() {
        "Projects (→ to navigate links)"
    } else {
        "Projects"
    };
    
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