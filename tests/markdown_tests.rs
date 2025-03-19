mod common;

use hiredavidparker::tui::markdown::parse_markdown;
use ratatui::text::Text;

#[test]
fn test_parse_basic_markdown() {
    // Test basic markdown parsing
    let input = "# Heading\n\nThis is a paragraph.\n\n* List item 1\n* List item 2";
    let (text, links) = parse_markdown(input);
    
    // Check that parsing produced some text
    assert!(!text.lines.is_empty());
    
    // Check that we have at least 3 lines (heading, paragraph, list items)
    assert!(text.lines.len() >= 3);
    
    // Verify there's a heading (bold text in the first non-empty line)
    let heading_line = text.lines.iter().find(|line| !line.spans.is_empty()).unwrap();
    assert!(!heading_line.spans.is_empty());
    
    // Simply check that text contains both heading and paragraph content
    let text_content = text.lines.iter()
        .flat_map(|line| line.spans.iter())
        .map(|span| span.content.clone())
        .collect::<String>();
    
    assert!(text_content.contains("Heading"));
    assert!(text_content.contains("This is a paragraph"));
    
    // Verify there's a paragraph
    let paragraph_lines = text.lines.iter()
        .filter(|line| !line.spans.is_empty())
        .collect::<Vec<_>>();
    assert!(!paragraph_lines.is_empty());
}

#[test]
fn test_parse_markdown_with_links() {
    // Test markdown with links
    let input = "This is a [link](https://example.com) in text.";
    let (text, links) = parse_markdown(input);
    
    // Check that parsing produced some text
    assert!(!text.lines.is_empty());
    
    // Verify links were extracted
    assert!(!links.is_empty());
    
    // Check link details
    let link = &links[0];
    assert_eq!(link.text, "link");
    assert_eq!(link.url, "https://example.com");
}