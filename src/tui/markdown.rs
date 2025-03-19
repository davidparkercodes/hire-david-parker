use crate::tui::models::Link;
use pulldown_cmark::{Event, Options, Parser, Tag};
use ratatui::{
    style::{Color, Modifier, Style},
    text::{Line, Span, Text},
};

/// Parse markdown text into Ratatui Text and extract hyperlinks
pub fn parse_markdown(content: &str) -> (Text<'static>, Vec<Link>) {
    let mut options = Options::empty();
    options.insert(Options::ENABLE_STRIKETHROUGH);
    
    let parser = Parser::new_ext(content, options);
    
    // Remove unused spans vector
    let mut lines: Vec<Line> = Vec::new();
    let mut current_line: Vec<Span> = Vec::new();
    let mut active_styles = Vec::new();
    let mut active_link_url: Option<String> = None;
    let mut links: Vec<Link> = Vec::new();
    let mut current_line_idx = 0;
    let mut current_column = 0;
    
    for event in parser {
        match event {
            Event::Start(tag) => {
                match tag {
                    Tag::Heading(_level, ..) => {
                        // Only apply bold modifier without color for maximum theme compatibility
                        active_styles.push(Style::default()
                            .add_modifier(Modifier::BOLD));
                            
                        // Add a blank line before headings (except at the very start)
                        if !current_line.is_empty() || !lines.is_empty() {
                            if !current_line.is_empty() {
                                lines.push(Line::from(current_line.clone()));
                                current_line.clear();
                                current_line_idx += 1;
                                current_column = 0;
                            }
                            lines.push(Line::from(Vec::new()));
                            current_line_idx += 1;
                        }
                    },
                    Tag::Paragraph => {
                        if !current_line.is_empty() {
                            lines.push(Line::from(current_line.clone()));
                            current_line.clear();
                            current_line_idx += 1;
                            current_column = 0;
                        }
                    },
                    Tag::Emphasis => {
                        active_styles.push(Style::default().add_modifier(Modifier::ITALIC));
                    },
                    Tag::Strong => {
                        active_styles.push(Style::default().add_modifier(Modifier::BOLD));
                    },
                    Tag::Strikethrough => {
                        active_styles.push(Style::default().add_modifier(Modifier::CROSSED_OUT));
                    },
                    Tag::Link(_link_type, url, _title) => {
                        active_link_url = Some(url.to_string());
                        active_styles.push(Style::default()
                            .fg(Color::Cyan)
                            .add_modifier(Modifier::UNDERLINED));
                    },
                    Tag::List(_) => {
                        if !current_line.is_empty() {
                            lines.push(Line::from(current_line.clone()));
                            current_line.clear();
                            current_line_idx += 1;
                            current_column = 0;
                        }
                    },
                    Tag::Item => {
                        current_line.push(Span::raw("• "));
                        current_column += 2;
                    },
                    _ => {}
                }
            },
            Event::End(tag) => {
                match tag {
                    Tag::Heading(..) => {
                        lines.push(Line::from(current_line.clone()));
                        current_line.clear();
                        current_line_idx += 1;
                        lines.push(Line::from(Vec::new())); // Add blank line after heading
                        current_line_idx += 1;
                        active_styles.pop();
                    },
                    Tag::Paragraph => {
                        if !current_line.is_empty() {
                            lines.push(Line::from(current_line.clone()));
                            current_line.clear();
                            current_line_idx += 1;
                            current_column = 0;
                        }
                        lines.push(Line::from(Vec::new())); // Add blank line after paragraph
                        current_line_idx += 1;
                    },
                    Tag::List(_) => {
                        if !current_line.is_empty() {
                            lines.push(Line::from(current_line.clone()));
                            current_line.clear();
                            current_line_idx += 1;
                            current_column = 0;
                        }
                        lines.push(Line::from(Vec::new())); // Add blank line after list
                        current_line_idx += 1;
                    },
                    Tag::Link(_, _, _) => {
                        active_styles.pop();
                        active_link_url = None;
                    },
                    Tag::Emphasis | Tag::Strong | Tag::Strikethrough => {
                        active_styles.pop();
                    },
                    Tag::Item => {
                        lines.push(Line::from(current_line.clone()));
                        current_line.clear();
                        current_line_idx += 1;
                        current_column = 0;
                    },
                    _ => {}
                }
            },
            Event::Text(text) => {
                let mut style = Style::default();
                for s in &active_styles {
                    style = style.patch(*s);
                }
                
                let text_str = text.to_string();
                let start_column = current_column;
                let end_column = start_column + text_str.len();
                
                // If we're inside a link, store the link information
                if let Some(url) = &active_link_url {
                    // Add all link URLs for processing
                    links.push(Link {
                        text: text_str.clone(),
                        url: url.clone(),
                        line: current_line_idx,
                        start_column,
                        end_column,
                    });
                }
                
                current_line.push(Span::styled(text_str, style));
                current_column = end_column;
            },
            Event::SoftBreak => {
                current_line.push(Span::raw(" "));
                current_column += 1;
            },
            Event::HardBreak => {
                lines.push(Line::from(current_line.clone()));
                current_line.clear();
                current_line_idx += 1;
                current_column = 0;
            },
            Event::Code(text) => {
                // Use only styling without color for inline code
                let style = Style::default()
                    .add_modifier(Modifier::BOLD);
                
                let text_str = text.to_string();
                let start_column = current_column;
                let end_column = start_column + text_str.len();
                
                current_line.push(Span::styled(text_str, style));
                current_column = end_column;
            },
            _ => {}
        }
    }
    
    // Add any remaining content
    if !current_line.is_empty() {
        lines.push(Line::from(current_line));
    }
    
    (Text::from(lines), links)
}