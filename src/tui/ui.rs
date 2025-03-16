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
        .style(Style::default().fg(Color::Cyan).add_modifier(Modifier::BOLD))
        .alignment(Alignment::Center)
        .block(Block::default().borders(Borders::ALL));
    f.render_widget(title, chunks[0]);

    // Create footer
    let footer_text = match app.display_mode {
        DisplayMode::Menu => "q: Quit | ↑/k: Up | ↓/j: Down | Enter: Select",
        _ => "q: Quit | ↑/k: Up | ↓/j: Down | Enter: Select | Esc: Return to Menu",
    };
    let footer = Paragraph::new(footer_text)
        .style(Style::default().fg(Color::DarkGray))
        .alignment(Alignment::Center)
        .block(Block::default().borders(Borders::ALL));
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
        DisplayMode::Menu => render_welcome(f, content_chunks[1]),
        DisplayMode::About => render_about(f, app, content_chunks[1]),
        DisplayMode::Skills => render_skills(f, app, content_chunks[1]),
        DisplayMode::Projects => render_projects(f, app, content_chunks[1]),
        DisplayMode::WhyWarp => render_why_warp(f, app, content_chunks[1]),
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
                Style::default().fg(Color::Yellow).add_modifier(Modifier::BOLD)
            } else {
                Style::default().fg(Color::White)
            };
            ListItem::new(Line::from(Span::styled(*item, style)))
        })
        .collect();

    let menu = List::new(items)
        .block(Block::default().title("Menu").borders(Borders::ALL))
        .highlight_style(Style::default().add_modifier(Modifier::BOLD));

    f.render_widget(menu, area);
}
/// Renders the welcome screen
fn render_welcome(f: &mut Frame, area: ratatui::layout::Rect) {
    let markdown_text = "# Welcome to my interactive resume!\n\n\
        Use the arrow keys to navigate the menu\n\
        and press **Enter** to select an option.\n\n\
        This TUI application demonstrates my **Rust skills**\n\
        and interest in *terminal applications* like **Warp**.";

    let text = parse_markdown(markdown_text);
    let instructions = Paragraph::new(text)
        .block(Block::default().title("Instructions").borders(Borders::ALL))
        .wrap(Wrap { trim: true });

    f.render_widget(instructions, area);
}

/// Renders the about section
fn render_about(f: &mut Frame, app: &App, area: ratatui::layout::Rect) {
    let text = parse_markdown(&app.about_content);
    let paragraph = Paragraph::new(text)
        .block(Block::default().title("About Me").borders(Borders::ALL))
        .wrap(Wrap { trim: true });
    f.render_widget(paragraph, area);
}

/// Renders the skills section
fn render_skills(f: &mut Frame, _app: &App, area: ratatui::layout::Rect) {
    let markdown_text = "## Technical Skills\n\n\
        * **Languages**: Rust, TypeScript, JavaScript, Python, Go\n\
        * **Frontend**: React, Vue.js, Web Technologies\n\
        * **Backend**: Node.js, Express, Actix Web, Tokio\n\
        * **Systems**: Linux, macOS, Docker, Kubernetes\n\
        * **Tools**: Git, GitHub, CI/CD, Terminal tools\n\
        \n\
        ## Terminal Expertise\n\n\
        * **Shell**: Bash, Zsh, Fish shell scripting\n\
        * **TUI Development**: Crossterm, Ratatui, ncurses\n\
        * **CLI Tools**: Created multiple command-line tools\n\
        * **Performance**: Optimizing terminal applications\n\
        * **UX**: Designing intuitive terminal interfaces";

    let text = parse_markdown(markdown_text);
    let paragraph = Paragraph::new(text)
        .block(Block::default().title("Skills").borders(Borders::ALL))
        .wrap(Wrap { trim: true });
    f.render_widget(paragraph, area);
}

/// Renders the projects section
fn render_projects(f: &mut Frame, _app: &App, area: ratatui::layout::Rect) {
    let markdown_text = "## Notable Projects\n\n\
        ### Terminal Dashboard (Rust)\n\
        A system monitoring dashboard with real-time stats, built with Rust and Ratatui.\n\
        \n\
        ### CLI Task Manager (Rust)\n\
        A fast, feature-rich task manager that works from the terminal.\n\
        \n\
        ### Code Snippet Manager (TypeScript)\n\
        VS Code extension for managing code snippets with 10k+ downloads.\n\
        \n\
        ### Terminal File Explorer (Rust)\n\
        A modern file explorer with vim-like keybindings and search capabilities.\n\
        \n\
        ### Weather CLI (Rust)\n\
        A simple weather app for the terminal with colorful output and forecast data.";

    let text = parse_markdown(markdown_text);
    let paragraph = Paragraph::new(text)
        .block(Block::default().title("Projects").borders(Borders::ALL))
        .wrap(Wrap { trim: true });
    f.render_widget(paragraph, area);
}

/// Renders the "Why Warp?" section
fn render_why_warp(f: &mut Frame, _app: &App, area: ratatui::layout::Rect) {
    let markdown_text = "## Why I Want to Join Warp\n\n\
        I've been a terminal power user for years, but I've always felt that\n\
        terminal technology hasn't evolved with modern UX expectations. When I\n\
        discovered Warp, I was thrilled to see a team tackling this problem\n\
        head-on.\n\
        \n\
        **What excites me about Warp:**\n\
        \n\
        * **Vision**: Reimagining the terminal for modern developers\n\
        * **Technology**: Rust's performance and safety for terminal applications\n\
        * **UX Focus**: Bringing modern interface design to the terminal\n\
        * **Innovation**: Not just incremental improvements, but rethinking fundamentals\n\
        \n\
        I'm enthusiastic about contributing to this mission and helping build\n\
        the next generation of terminal experiences that developers love to use.";

    let text = parse_markdown(markdown_text);
    let paragraph = Paragraph::new(text)
        .block(Block::default().title("Why Warp?").borders(Borders::ALL))
        .wrap(Wrap { trim: true });
    f.render_widget(paragraph, area);
}