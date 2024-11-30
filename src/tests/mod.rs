#[cfg(test)]
mod tests {
    use crate::{MarkdownElement, MarkdownParser};

    #[test]
    fn test_simple_header() {
        let content = "# Hello World".to_string();
        let parser = MarkdownParser::new(content);
        let elements = parser.parse();
        assert_eq!(elements.len(), 1);
        assert_eq!(
            elements[0],
            MarkdownElement::Header {
                level: 1,
                content: "Hello World".to_string()
            }
        );
    }

    #[test]
    fn test_simple_paragraph() {
        let content = "This is a paragraph".to_string();
        let parser = MarkdownParser::new(content);
        let elements = parser.parse();
        assert_eq!(elements.len(), 1);
        assert_eq!(
            elements[0],
            MarkdownElement::Paragraph("This is a paragraph".to_string())
        );
    }

    #[test]
    fn test_header_and_paragraph() {
        let content = "# Header\nThis is a paragraph".to_string();
        let parser = MarkdownParser::new(content);
        let elements = parser.parse();
        assert_eq!(elements.len(), 2);
        assert_eq!(
            elements[0],
            MarkdownElement::Header {
                level: 1,
                content: "Header".to_string()
            }
        );
        assert_eq!(
            elements[1],
            MarkdownElement::Paragraph("This is a paragraph".to_string())
        );
    }

    #[test]
    fn test_multiple_paragraphs() {
        let content = "First paragraph\n\nSecond paragraph".to_string();
        let parser = MarkdownParser::new(content);
        let elements = parser.parse();
        assert_eq!(elements.len(), 2);
        assert_eq!(
            elements[0],
            MarkdownElement::Paragraph("First paragraph".to_string())
        );
        assert_eq!(
            elements[1],
            MarkdownElement::Paragraph("Second paragraph".to_string())
        );
    }

    #[test]
    fn test_mixed_content() {
        let content = "# Header\nPara 1\n\n## Header 2\nPara 2".to_string();
        let parser = MarkdownParser::new(content);
        let elements = parser.parse();
        assert_eq!(elements.len(), 4);
        assert_eq!(
            elements[0],
            MarkdownElement::Header {
                level: 1,
                content: "Header".to_string()
            }
        );
        assert_eq!(elements[1], MarkdownElement::Paragraph("Para 1".to_string()));
        assert_eq!(
            elements[2],
            MarkdownElement::Header {
                level: 2,
                content: "Header 2".to_string()
            }
        );
        assert_eq!(elements[3], MarkdownElement::Paragraph("Para 2".to_string()));
    }
}