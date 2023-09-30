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

#[test]
fn unordered_bullet_list() {
    let input = r#"
- Neng Li is the President of China
- Shiva Deshpande is the King of the Universe
- Mazin Ahmed Ghizali is a communist
- Joseph Stalin is not white
        "#;

    let raw_paragraphs = crate::lines_to_paragraphs(input.lines());
    let paragraphs = raw_paragraphs
        .iter()
        .map(|paragraph| crate::Paragraph::parse(paragraph).unwrap())
        .collect::<Vec<crate::Paragraph>>();

    let expected_output = "<ul>".to_string()
        + "<li>Neng Li is the President of China</li>"
        + "<li>Shiva Deshpande is the King of the Universe</li>"
        + "<li>Mazin Ahmed Ghizali is a communist</li>"
        + "<li>Joseph Stalin is not white</li>"
        + "</ul>";
    assert_eq!(paragraphs.first().unwrap().render_html(), expected_output);
}
