use ratatui::{
    layout::{Constraint, Direction, Layout, Alignment},
    style::{Color, Modifier, Style},
    text::{Line, Span},
    widgets::{Block, Borders, List, ListItem, Paragraph, Wrap},
    Frame,
};

use super::app::{App, DisplayMode, TransitionState, WipeDirection};
use super::markdown::parse_markdown;

/// Renders the user interface widgets
pub fn render(f: &mut Frame, app: &App) {
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
    
    // Handle rendering based on transition state
    match app.transition {
        TransitionState::None => {
            // Normal rendering when no transition is active
            render_content(f, app, app.display_mode, content_chunks[1]);
        }
        TransitionState::Wipe { progress, direction } => {
            // Render wipe transition
            render_wipe_transition(f, app, content_chunks[1], progress, direction);
        }
    }
}

/// Renders the appropriate content based on display mode
fn render_content(f: &mut Frame, app: &App, mode: DisplayMode, area: ratatui::layout::Rect) {
    match mode {
        DisplayMode::Menu => render_welcome(f, app, area),
        DisplayMode::About => render_about(f, app, area),
        DisplayMode::Skills => render_skills(f, app, area),
        DisplayMode::Projects => render_projects(f, app, area),
        DisplayMode::WhyWarp => render_why_warp(f, app, area),
    }
}

/// Renders a wipe transition between two content sections
fn render_wipe_transition(
    f: &mut Frame,
    app: &App,
    area: ratatui::layout::Rect,
    progress: u8,
    direction: WipeDirection,
) {
    // Calculate the dividing point based on progress and direction
    let dividing_point = match direction {
        WipeDirection::LeftToRight => {
            // Progress from 0 (left) to 100 (right)
            (area.width as f32 * (progress as f32 / 100.0)) as u16
        }
        WipeDirection::RightToLeft => {
            // Progress from 100 (right) to 0 (left)
            (area.width as f32 * (1.0 - progress as f32 / 100.0)) as u16
        }
    };

    // Create two sub-areas separated by the dividing line
    let left_area = ratatui::layout::Rect {
        x: area.x,
        y: area.y,
        width: dividing_point,
        height: area.height,
    };

    let right_area = ratatui::layout::Rect {
        x: area.x + dividing_point,
        y: area.y,
        width: area.width - dividing_point,
        height: area.height,
    };

    // Determine which content to render in which section
    match direction {
        WipeDirection::LeftToRight => {
            // Left area shows new content, right area shows old content
            if left_area.width > 0 {
                render_content(f, app, app.display_mode, left_area);
            }
            if right_area.width > 0 {
                render_content(f, app, app.previous_mode, right_area);
            }
        }
        WipeDirection::RightToLeft => {
            // Left area shows old content, right area shows new content
            if left_area.width > 0 {
                render_content(f, app, app.previous_mode, left_area);
            }
            if right_area.width > 0 {
                render_content(f, app, app.display_mode, right_area);
            }
        }
    }
}

/// Renders the menu sidebar (always visible)
fn render_menu_sidebar(f: &mut Frame, app: &App, area: ratatui::layout::Rect) {
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
fn render_welcome(f: &mut Frame, app: &App, area: ratatui::layout::Rect) {
    let text = parse_markdown(&app.welcome_content);
    let instructions = Paragraph::new(text)
        .block(Block::default().title("Instructions").borders(Borders::ALL).border_style(Style::default().fg(Color::Blue)))
        .wrap(Wrap { trim: true });

    f.render_widget(instructions, area);
}

/// Renders the about section
fn render_about(f: &mut Frame, app: &App, area: ratatui::layout::Rect) {
    let text = parse_markdown(&app.about_content);
    let paragraph = Paragraph::new(text)
        .block(Block::default().title("About Me").borders(Borders::ALL).border_style(Style::default().fg(Color::Blue)))
        .wrap(Wrap { trim: true });
    f.render_widget(paragraph, area);
}

/// Renders the skills section
fn render_skills(f: &mut Frame, app: &App, area: ratatui::layout::Rect) {
    let text = parse_markdown(&app.skills_content);
    let paragraph = Paragraph::new(text)
        .block(Block::default().title("Skills").borders(Borders::ALL).border_style(Style::default().fg(Color::Blue)))
        .wrap(Wrap { trim: true });
    f.render_widget(paragraph, area);
}

/// Renders the projects section
fn render_projects(f: &mut Frame, app: &App, area: ratatui::layout::Rect) {
    let text = parse_markdown(&app.projects_content);
    let paragraph = Paragraph::new(text)
        .block(Block::default().title("Projects").borders(Borders::ALL).border_style(Style::default().fg(Color::Blue)))
        .wrap(Wrap { trim: true });
    f.render_widget(paragraph, area);
}

/// Renders the "Why Warp?" section
fn render_why_warp(f: &mut Frame, app: &App, area: ratatui::layout::Rect) {
    let text = parse_markdown(&app.why_warp_content);
    let paragraph = Paragraph::new(text)
        .block(Block::default().title("Why Warp?").borders(Borders::ALL).border_style(Style::default().fg(Color::Blue)))
        .wrap(Wrap { trim: true });
    f.render_widget(paragraph, area);
}