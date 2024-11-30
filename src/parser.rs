use crate::element::MarkdownElement;

pub struct MarkdownParser {
    content: String,
}

impl MarkdownParser {
    pub fn new(content: String) -> Self {
        Self { content }
    }

    pub fn parse(&self) -> Vec<MarkdownElement> {
        let mut elements = Vec::new();
        let mut current_paragraph = String::new();
        for lines in self.content.lines() {
            let trim_lines = lines.trim();

            if trim_lines.is_empty() {
                if !current_paragraph.is_empty() {
                    elements.push(MarkdownElement::Paragraph(current_paragraph.trim().to_string()));
                    current_paragraph.clear();
                }
                continue;
            }
            if trim_lines.starts_with('#') {

                if !current_paragraph.is_empty() {
                    elements.push(MarkdownElement::Paragraph(current_paragraph.trim().to_string()));
                    current_paragraph.clear();
                }

                if let Some(header) = self.parse_header(trim_lines) {
                    elements.push(header);
                }
            } else {
                if !current_paragraph.is_empty() {
                    current_paragraph.push(' ');
                }
                current_paragraph.push_str(trim_lines);
            }
        }

        if !current_paragraph.is_empty() {
            elements.push(MarkdownElement::Paragraph(current_paragraph.trim().to_string()));
        }
        elements
    }

    fn parse_header(&self, line: &str) -> Option<MarkdownElement> {
        let mut level: usize = 0;
        for char in line.chars() {
            if char == '#' {
                level += 1;
            } else {
                break;
            }
        }
        if level > 0 && level <= 6 {
            Some(MarkdownElement::Header {
                level: level as u8,
                content: line[level..].trim().to_string(),
            })
        } else {
            None
        }
    }
}