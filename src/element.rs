#[derive(Debug, PartialEq)]
pub enum MarkdownElement {
    Header {
        level: u8,
        content: String,
    },
    Paragraph(String),
}