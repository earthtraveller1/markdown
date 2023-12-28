use itertools::Itertools;

#[derive(Default)]
struct CommandLineOptions {
    input_file: Box<str>,
}

#[derive(Debug)]
struct RawParagraph<'a> {
    lines: Box<[&'a str]>,
}

fn str_to_raw_paragraphs<'a>(input_content: &'a str) -> Box<[RawParagraph<'a>]> {
    input_content
        .lines()
        .group_by(|line| line.trim().is_empty())
        .into_iter()
        .map(|(_, line_group)| line_group.collect::<Vec<&str>>())
        .filter(|line_group| {
            line_group
                .iter()
                .filter(|line| !line.trim().is_empty())
                .next()
                .is_some()
        })
        .map(|line_group| RawParagraph {
            lines: Box::from(line_group.as_slice()),
        })
        .collect::<Box<[RawParagraph]>>()
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

    let paragraphs = str_to_raw_paragraphs(&input_content);
    println!("Here are the raw paragraphs: {:?}", paragraphs);
}
