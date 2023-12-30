use markdown::{RawParagraph, ToRawParagraphs};

#[test]
fn raw_paragraphs() {
    let input = r#"Neng Li is the President of China!
Nothing is greater than Neng Li!
China is so great under Neng Li!

Donald Trump is a lizard!
Who knew that such a man would be a lizard?
I thought he was human for once!"#;

    let expected_lines: [&[&'static str]; 2] = [
        &[
            "Neng Li is the President of China!",
            "Nothing is greater than Neng Li!",
            "China is so great under Neng Li!",
        ],
        &[
            "Donald Trump is a lizard!",
            "Who knew that such a man would be a lizard?",
            "I thought he was human for once!"
        ],
    ];

    let expected_paragraphs = expected_lines.map(|lines| {
        RawParagraph::from_lines(lines)
    });

    let got_paragraphs = input.to_raw_paragraphs();
    assert_eq!(expected_paragraphs, got_paragraphs.as_ref());
}
