mod element;
mod parser;

pub use element::MarkdownElement;
pub use parser::MarkdownParser;

#[cfg(test)]
mod tests;