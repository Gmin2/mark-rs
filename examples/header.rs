use mark_rs::MarkdownParser;

fn main() {
    let markdown = r#"# Header 1
## Header 2
### Header 3
Not a header
#### Header 4"#
        .to_string();

    let parser = MarkdownParser::new(markdown);
    let elements = parser.parse();

    for element in elements {
        println!("{:?}", element);
    }
}
