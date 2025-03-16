use pulldown_cmark::{Event, Options, Parser, Tag};
use ratatui::{
    style::{Color, Modifier, Style},
    text::{Line, Span, Text},
};

/// Parse markdown text into Ratatui Text
pub fn parse_markdown(content: &str) -> Text<'static> {
    let mut options = Options::empty();
    options.insert(Options::ENABLE_STRIKETHROUGH);
    
    let parser = Parser::new_ext(content, options);
    
    // Remove unused spans vector
    let mut lines: Vec<Line> = Vec::new();
    let mut current_line: Vec<Span> = Vec::new();
    let mut active_styles = Vec::new();
    
    for event in parser {
        match event {
            Event::Start(tag) => {
                match tag {
                    Tag::Heading(level, ..) => {
                        // Set styles based on heading level
                        let level_num = level as u8;
                        // Use level_num to potentially set different colors per level
                        let heading_color = match level_num {
                            1 => Color::Cyan,
                            2 => Color::LightCyan,
                            _ => Color::Blue,
                        };
                        active_styles.push(Style::default()
                            .fg(heading_color)
                            .add_modifier(Modifier::BOLD));
                            
                        // Add a blank line before headings (except at the very start)
                        if !current_line.is_empty() || !lines.is_empty() {
                            if !current_line.is_empty() {
                                lines.push(Line::from(current_line.clone()));
                                current_line.clear();
                            }
                            lines.push(Line::from(Vec::new()));
                        }
                    },
                    Tag::Paragraph => {
                        if !current_line.is_empty() {
                            lines.push(Line::from(current_line.clone()));
                            current_line.clear();
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
                    Tag::List(_) => {
                        if !current_line.is_empty() {
                            lines.push(Line::from(current_line.clone()));
                            current_line.clear();
                        }
                    },
                    Tag::Item => {
                        current_line.push(Span::raw("• "));
                    },
                    _ => {}
                }
            },
            Event::End(tag) => {
                match tag {
                    Tag::Heading(..) => {
                        lines.push(Line::from(current_line.clone()));
                        current_line.clear();
                        lines.push(Line::from(Vec::new())); // Add blank line after heading
                        active_styles.pop();
                    },
                    Tag::Paragraph => {
                        if !current_line.is_empty() {
                            lines.push(Line::from(current_line.clone()));
                            current_line.clear();
                        }
                        lines.push(Line::from(Vec::new())); // Add blank line after paragraph
                    },
                    Tag::Emphasis | Tag::Strong | Tag::Strikethrough => {
                        active_styles.pop();
                    },
                    Tag::Item => {
                        lines.push(Line::from(current_line.clone()));
                        current_line.clear();
                    },
                    _ => {}
                }
            },
            Event::Text(text) => {
                let mut style = Style::default();
                for s in &active_styles {
                    style = style.patch(*s);
                }
                current_line.push(Span::styled(text.to_string(), style));
            },
            Event::SoftBreak => {
                current_line.push(Span::raw(" "));
            },
            Event::HardBreak => {
                lines.push(Line::from(current_line.clone()));
                current_line.clear();
            },
            Event::Code(text) => {
                let style = Style::default()
                    .fg(Color::Yellow)
                    .add_modifier(Modifier::BOLD);
                current_line.push(Span::styled(text.to_string(), style));
            },
            _ => {}
        }
    }
    
    // Add any remaining content
    if !current_line.is_empty() {
        lines.push(Line::from(current_line));
    }
    
    Text::from(lines)
}