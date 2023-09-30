#[test]
fn basic_sanity_test() {
    let input = r#"
# Title

## Second Title

### Third Title

Regular Paragraph
        "#;

    let raw_paragraphs = crate::lines_to_paragraphs(input.lines());
    let paragraphs = raw_paragraphs
        .iter()
        .map(|paragraph| crate::Paragraph::parse(paragraph).unwrap())
        .collect::<Vec<crate::Paragraph>>();

    assert_eq!(paragraphs[0].render_html(), "<h1>Title</h1>");
    assert_eq!(paragraphs[1].render_html(), "<h2>Second Title</h2>");
    assert_eq!(paragraphs[2].render_html(), "<h3>Third Title</h3>");
    assert_eq!(paragraphs[3].render_html(), "<p>Regular Paragraph</p>");
}
