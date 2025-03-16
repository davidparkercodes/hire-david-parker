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

/// Renders a true wipe transition between two content sections
fn render_wipe_transition(
    f: &mut Frame,
    app: &App,
    area: ratatui::layout::Rect,
    progress: u8,
    direction: WipeDirection,
) {
    // First, let's get the new content and title
    let current_mode = app.display_mode;
    let previous_mode = app.previous_mode;
    
    // Get content for both states
    let (new_title, new_content) = match current_mode {
        DisplayMode::Menu => ("Instructions", parse_markdown(&app.welcome_content)),
        DisplayMode::About => ("About Me", parse_markdown(&app.about_content)),
        DisplayMode::Skills => ("Skills", parse_markdown(&app.skills_content)),
        DisplayMode::Projects => ("Projects", parse_markdown(&app.projects_content)),
        DisplayMode::WhyWarp => ("Why Warp?", parse_markdown(&app.why_warp_content)),
    };
    
    let (old_title, old_content) = match previous_mode {
        DisplayMode::Menu => ("Instructions", parse_markdown(&app.welcome_content)),
        DisplayMode::About => ("About Me", parse_markdown(&app.about_content)),
        DisplayMode::Skills => ("Skills", parse_markdown(&app.skills_content)),
        DisplayMode::Projects => ("Projects", parse_markdown(&app.projects_content)),
        DisplayMode::WhyWarp => ("Why Warp?", parse_markdown(&app.why_warp_content)),
    };
    
    // Calculate how much of the wipe effect should be complete
    let wipe_percent = progress as f32 / 100.0;
    
    // Create a block for content with the appropriate title
    // During transition, we show the new title
    let block = Block::default()
        .title(new_title)
        .borders(Borders::ALL)
        .border_style(Style::default().fg(Color::Blue));
    
    // Render the block
    f.render_widget(block.clone(), area);
    
    // Get the inner area for content
    let inner_area = block.inner(area);
    
    // For a true wipe effect, we'll render line by line
    // First get both old and new content as lines
    let old_lines = old_content.lines;
    let new_lines = new_content.lines;
    
    // Get maximum number of lines we can display
    let max_lines = inner_area.height as usize;
    
    // Create a new set of lines for our wipe effect
    let mut wipe_lines = Vec::new();
    
    // Determine how many characters to reveal based on direction and progress
    let char_reveal_count = match direction {
        WipeDirection::LeftToRight => {
            // Left to right wipe reveals characters from the left
            (inner_area.width as f32 * wipe_percent) as usize
        }
        WipeDirection::RightToLeft => {
            // Right to left wipe reveals characters from the right
            (inner_area.width as f32 * wipe_percent) as usize
        }
    };
    
    // Loop through visible lines
    for i in 0..max_lines {
        // Get old line if available, otherwise use empty line
        let old_line = old_lines.get(i).cloned().unwrap_or_else(|| Line::default());
        // Get new line if available, otherwise use empty line
        let new_line = new_lines.get(i).cloned().unwrap_or_else(|| Line::default());
        
        // For a wipe effect, we'll construct a hybrid line:
        // - For LeftToRight: start with new content up to reveal point, then old content
        // - For RightToLeft: start with old content, then switch to new content from reveal point
        
        match direction {
            WipeDirection::LeftToRight => {
                // For left to right, we determine how much of the content to show
                // based on the progress
                let line_width = inner_area.width as usize;
                
                // If we're at 0% progress, show all old content
                // If we're at 100% progress, show all new content
                if char_reveal_count == 0 || progress == 0 {
                    // Just show old content
                    wipe_lines.push(old_line);
                } else if char_reveal_count >= line_width || progress == 100 {
                    // Just show new content
                    wipe_lines.push(new_line);
                } else {
                    // Combine content - create a new line with a mix of old and new
                    // For simplicity, we'll convert both to strings and do character-based combining
                    
                    // Get the text content of the lines
                    let old_text = old_line.spans.iter()
                        .map(|span| span.content.as_ref())
                        .collect::<String>();
                        
                    let new_text = new_line.spans.iter()
                        .map(|span| span.content.as_ref())
                        .collect::<String>();
                    
                    // Create a combined text where the left part comes from new text
                    // and the right part comes from old text
                    let mut combined_text = String::new();
                    
                    // Add characters from new text (up to char_reveal_count)
                    let new_chars = new_text.chars().take(char_reveal_count).collect::<String>();
                    combined_text.push_str(&new_chars);
                    
                    // Fill the rest from old text
                    if new_chars.len() < line_width {
                        let remaining = line_width - new_chars.len();
                        if old_text.len() >= char_reveal_count {
                            let old_chars = old_text.chars().skip(char_reveal_count).take(remaining).collect::<String>();
                            combined_text.push_str(&old_chars);
                        }
                    }
                    
                    // Create a new line from the combined text
                    wipe_lines.push(Line::from(combined_text));
                }
            }
            WipeDirection::RightToLeft => {
                // For right to left, we determine how much of the content to show
                // based on the inverse of progress (100 - progress)
                let line_width = inner_area.width as usize;
                
                // Create a new line based on the old line, but with spaces
                // in the revealed area (which will be filled in later)
                let mut visible_chars = line_width - char_reveal_count;
                
                // If we're at 0% progress, show all old content
                // If we're at 100% progress, show all new content
                if visible_chars >= line_width || progress == 0 {
                    // Just show old content
                    wipe_lines.push(old_line);
                } else if visible_chars == 0 || progress == 100 {
                    // Just show new content
                    wipe_lines.push(new_line);
                } else {
                    // Combine content - create a new line with a mix of old and new
                    // For simplicity, we'll convert both to strings and do character-based combining
                    
                    // Get the text content of the lines
                    let old_text = old_line.spans.iter()
                        .map(|span| span.content.as_ref())
                        .collect::<String>();
                        
                    let new_text = new_line.spans.iter()
                        .map(|span| span.content.as_ref())
                        .collect::<String>();
                    
                    // Create a combined text where the left part comes from old text
                    // and the right part comes from new text
                    let mut combined_text = String::new();
                    
                    // Add characters from old text (up to visible_chars)
                    let old_chars = old_text.chars().take(visible_chars).collect::<String>();
                    combined_text.push_str(&old_chars);
                    
                    // Fill the rest from new text
                    if old_chars.len() < line_width {
                        let remaining = line_width - old_chars.len();
                        if remaining <= new_text.len() {
                            let new_chars = new_text.chars().take(remaining).collect::<String>();
                            combined_text.push_str(&new_chars);
                        }
                    }
                    
                    // Create a new line from the combined text
                    wipe_lines.push(Line::from(combined_text));
                }
            }
        }
    }
    
    // Create a paragraph with our wipe effect lines
    let wipe_para = Paragraph::new(wipe_lines)
        .wrap(Wrap { trim: true });
    
    // Render the wipe effect
    f.render_widget(wipe_para, inner_area);
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