mod common;

use hiredavidparker::tui::markdown::parse_markdown;
use ratatui::style::Modifier;

#[test]
fn test_parse_basic_markdown() {
    // Test basic markdown parsing
    let input = "# Heading\n\nThis is a paragraph.\n\n* List item 1\n* List item 2";
    let (text, _links) = parse_markdown(input);
    
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
    assert_eq!(link.line, 0);
    assert_eq!(link.start_column, 10);
    assert_eq!(link.end_column, 14);
}

#[test]
fn test_parse_markdown_with_text_styles() {
    // Test various text styling elements
    let input = "Normal *italic* **bold** ~~strikethrough~~ text";
    let (text, _) = parse_markdown(input);
    
    // Text with styling will have some content lines plus blank lines
    assert!(text.lines.len() > 1);
    
    let line = &text.lines[0];
    // Don't assert exact span count as it may vary
    
    // Find the styled spans
    let italic_span = line.spans.iter().find(|span| {
        span.content == "italic" && 
        span.style.add_modifier.contains(Modifier::ITALIC)
    });
    let bold_span = line.spans.iter().find(|span| {
        span.content == "bold" && 
        span.style.add_modifier.contains(Modifier::BOLD)
    });
    let strikethrough_span = line.spans.iter().find(|span| {
        span.content == "strikethrough" && 
        span.style.add_modifier.contains(Modifier::CROSSED_OUT)
    });
    
    assert!(italic_span.is_some(), "Failed to find italic text");
    assert!(bold_span.is_some(), "Failed to find bold text");
    assert!(strikethrough_span.is_some(), "Failed to find strikethrough text");
}

#[test]
fn test_parse_markdown_with_lists() {
    // Test unordered lists
    let input = "* Item 1\n* Item 2\n* Item 3";
    let (text, _) = parse_markdown(input);
    
    // Count bullet points
    let bullet_count = text.lines.iter()
        .flat_map(|line| line.spans.iter())
        .filter(|span| span.content == "• ")
        .count();
    
    assert_eq!(bullet_count, 3, "There should be 3 bullet points");
    
    // Check content for each list item
    let items_content: Vec<String> = text.lines.iter()
        .filter(|line| !line.spans.is_empty())
        .map(|line| {
            line.spans.iter()
                .map(|span| span.content.clone())
                .collect::<String>()
        })
        .collect();
    
    assert_eq!(items_content.len(), 3, "There should be 3 list items");
    assert!(items_content[0].contains("Item 1"));
    assert!(items_content[1].contains("Item 2"));
    assert!(items_content[2].contains("Item 3"));
}

#[test]
fn test_parse_markdown_with_code() {
    // Test inline code
    let input = "This is `inline code` in text.";
    let (text, _) = parse_markdown(input);
    
    // Should have one line with multiple spans
    assert!(!text.lines.is_empty());
    
    // Find the code span
    let code_span = text.lines[0].spans.iter().find(|span| {
        span.content == "inline code" && 
        span.style.add_modifier.contains(Modifier::BOLD)
    });
    
    assert!(code_span.is_some(), "Failed to find styled inline code");
}

#[test]
fn test_parse_markdown_with_line_breaks() {
    // Test hard and soft breaks
    let input = "Line 1\nLine 2\n\nNew paragraph";
    let (text, _) = parse_markdown(input);
    
    // Content should be spread across multiple lines
    let content_lines: Vec<_> = text.lines.iter()
        .filter(|line| !line.spans.is_empty())
        .collect();
    
    assert!(content_lines.len() >= 2, "Should have at least two non-empty lines");
    
    // Check line contents
    let line_texts: Vec<String> = content_lines.iter()
        .map(|line| {
            line.spans.iter()
                .map(|span| span.content.clone())
                .collect::<String>()
        })
        .collect();
    
    assert!(line_texts.iter().any(|text| text.contains("Line 1")));
    assert!(line_texts.iter().any(|text| text.contains("New paragraph")));
}

#[test]
fn test_parse_markdown_with_empty_input() {
    // Test empty input
    let input = "";
    let (text, links) = parse_markdown(input);
    
    // Should produce empty text and no links
    assert!(links.is_empty(), "Empty input should produce no links");
    
    // May contain an empty line
    let non_empty_lines = text.lines.iter()
        .filter(|line| !line.spans.is_empty())
        .count();
    
    assert_eq!(non_empty_lines, 0, "Empty input should not produce any content");
}

#[test]
fn test_parse_markdown_with_complex_nesting() {
    // Test complex nesting
    let input = "# Heading with *italic* and **bold**\n\n* List item with [link](https://example.com)\n* **Bold item** with `code`";
    let (text, links) = parse_markdown(input);
    
    // Verify link was extracted
    assert_eq!(links.len(), 1);
    assert_eq!(links[0].text, "link");
    
    // Check for styled content in headings and lists
    let styled_spans_count = text.lines.iter()
        .flat_map(|line| line.spans.iter())
        .filter(|span| span.style.add_modifier != Modifier::empty())
        .count();
    
    assert!(styled_spans_count > 3, "Complex markdown should produce multiple styled spans");
    
    // Find a line with a bullet point
    let bullet_line = text.lines.iter()
        .find(|line| {
            line.spans.iter().any(|span| span.content == "• ")
        });
    
    assert!(bullet_line.is_some(), "No bullet point line found in complex markdown");
}

#[test]
fn test_paragraph_spacing() {
    // Test paragraph spacing
    let input = "Paragraph 1.\n\nParagraph 2.\n\nParagraph 3.";
    let (text, _) = parse_markdown(input);
    
    // Count non-empty lines
    let non_empty_lines = text.lines.iter()
        .filter(|line| !line.spans.is_empty())
        .count();
    
    // Count empty lines
    let empty_lines = text.lines.iter()
        .filter(|line| line.spans.is_empty())
        .count();
    
    assert_eq!(non_empty_lines, 3, "Should have 3 paragraphs");
    assert!(empty_lines >= 2, "Should have empty lines between paragraphs");
    
    // Extract paragraph texts
    let paragraph_texts: Vec<String> = text.lines.iter()
        .filter(|line| !line.spans.is_empty())
        .map(|line| {
            line.spans.iter()
                .map(|span| span.content.clone())
                .collect::<String>()
        })
        .collect();
    
    assert_eq!(paragraph_texts.len(), 3);
    assert_eq!(paragraph_texts[0], "Paragraph 1.");
    assert_eq!(paragraph_texts[1], "Paragraph 2.");
    assert_eq!(paragraph_texts[2], "Paragraph 3.");
}