use itertools::Itertools;
use markdown::{LineParagraph,ToRawParagraphs};

#[derive(Default)]
struct CommandLineOptions {
    input_file: Box<str>,
}

fn main() {
    let options = std::env::args().tuple_windows().fold(
        CommandLineOptions::default(),
        |mut acc, (first, last)| {
            if first == "-i" {
                acc.input_file = last.clone().into_boxed_str();
            }
            return acc;
        },
    );

    if options.input_file.is_empty() {
        eprintln!("[ERROR]: No input file specified.");
        std::process::exit(1);
    }

    let input_content = match std::fs::read_to_string(options.input_file.as_ref()) {
        Ok(x) => x,
        Err(error) => {
            eprintln!("[ERROR]: Failed to load the input file. Error {}", error);
            std::process::exit(1);
        }
    };

    let raw_paragraphs = input_content.as_str().to_raw_paragraphs();
    println!("Here are the raw paragraphs: {:?}", raw_paragraphs);

    let line_paragraphs = raw_paragraphs
        .iter()
        .map(|raw_paragraph| raw_paragraph.to_line_paragraph())
        .collect::<Box<[LineParagraph]>>();
    println!("Here are the line paragraphs: {:?}", line_paragraphs);
}
