#[cfg(test)]
mod tests;

use std::io::{BufWriter, Read, Write};

enum Input {
    File(std::fs::File),
    Stdin,
}

impl Read for Input {
    fn read(&mut self, buf: &mut [u8]) -> Result<usize, std::io::Error> {
        match self {
            Input::File(file) => file.read(buf),
            Input::Stdin => std::io::stdin().read(buf),
        }
    }
}

enum Output {
    File(std::fs::File),
    Stdout,
}

impl Write for Output {
    fn write(&mut self, buf: &[u8]) -> std::io::Result<usize> {
        match self {
            Output::File(file) => file.write(buf),
            Output::Stdout => std::io::stdout().write(buf),
        }
    }

    fn flush(&mut self) -> std::io::Result<()> {
        match self {
            Output::File(file) => file.flush(),
            Output::Stdout => std::io::stdout().flush(),
        }
    }
}

struct CmdOptions {
    input: Input,
    output: Output,
}

impl CmdOptions {
    fn get() -> Result<CmdOptions, ()> {
        let mut cmd_args = std::env::args();
        let mut cmd_options = CmdOptions {
            input: Input::Stdin,
            output: Output::Stdout,
        };

        // The first command line argument would be the program, so we have to first advance onto
        // the second one.
        cmd_args.next();

        while let Some(arg) = cmd_args.next() {
            if arg == "-o" || arg == "--output" {
                match cmd_args.next() {
                    Some(path) => {
                        cmd_options.output = Output::File({
                            let file = std::fs::File::create(path.as_str());
                            match file {
                                Ok(file) => file,
                                Err(_) => {
                                    eprintln!("[ERROR]: Could not open '{}' for writing.", path);
                                    return Err(());
                                }
                            }
                        });
                    }
                    None => {
                        eprintln!("[ERROR]: Expected filename after '{}'", arg);
                        return Err(());
                    }
                }
            } else {
                cmd_options.input = {
                    match std::fs::File::open(arg.as_str()) {
                        Ok(file) => Input::File(file),
                        Err(_) => {
                            eprintln!("[ERROR]: Could not open '{}'", arg);
                            return Err(());
                        }
                    }
                }
            }
        }

        Ok(cmd_options)
    }
}

#[derive(Debug)]
enum Line {
    Normal(String),
    Heading { level: u8, content: String },
}

impl Line {
    fn render_html(&self) -> String {
        match self {
            Line::Normal(content) => content.clone(),
            Line::Heading { level, content } => format!("<h{}>{}</h{}>", level, content, level),
        }
    }
}

#[derive(Debug)]
struct ListItem(String);

#[derive(Debug)]
enum Paragraph {
    Normal(Vec<Line>),
    UnorderedList(Vec<ListItem>),
}

impl Paragraph {
    fn parse(raw_paragraph: &Vec<String>) -> Result<Paragraph, &'static str> {
        let first_line = raw_paragraph.first();
        match first_line {
            None => return Err("Empty paragraph!"),
            Some(first_line) => {
                if first_line.starts_with("- ") {
                    Ok(Paragraph::UnorderedList({
                        let mut items = Vec::new();

                        for raw_line in raw_paragraph {
                            match raw_line.strip_prefix("- ") {
                                None => {
                                    return Err("Cannot have normal line in list paragraph.");
                                }
                                Some(line) => {
                                    items.push(ListItem(line.to_string()));
                                }
                            }
                        }

                        items
                    }))
                } else {
                    Ok(Paragraph::Normal(
                        raw_paragraph
                            .iter()
                            .map(|raw_line| {
                                let hash_counts: u8 = raw_line
                                    .chars()
                                    .take_while(|c| *c == '#')
                                    .count()
                                    .try_into()
                                    .unwrap();

                                if hash_counts == 0 {
                                    return Line::Normal(raw_line.to_owned());
                                }

                                if raw_line.chars().nth(hash_counts.into()) != Some(' ') {
                                    return Line::Normal(raw_line.to_owned());
                                }

                                return Line::Heading {
                                    level: hash_counts,
                                    content: raw_line
                                        .split_at(hash_counts.into())
                                        .1
                                        .trim()
                                        .to_string(),
                                };
                            })
                            .collect(),
                    ))
                }
            }
        }
    }

    fn render_html(&self) -> String {
        match self {
            Paragraph::Normal(lines) => {
                format!(
                    "<p>{}</p>",
                    lines
                        .iter()
                        .fold(String::new(), |acc, line| acc + line.render_html().as_str())
                )
            }
            Paragraph::UnorderedList(list_items) => {
                format!(
                    "<ul>{}</ul>",
                    list_items.iter().fold(String::new(), |acc, item| {
                        format!("{}<li>{}</li>", acc, item.0)
                    })
                )
            }
        }
    }
}

fn lines_to_paragraphs(lines: std::str::Lines) -> Vec<Vec<String>> {
    lines
        .fold(vec![vec!["".to_string()]], |mut acc, line| {
            if line.is_empty() {
                acc.push(Vec::new());
            } else {
                acc.last_mut().unwrap().push(line.to_string());
            }

            acc
        })
        .iter()
        .map(|paragraph| {
            paragraph
                .iter()
                .map(|line| line.trim().to_string())
                .filter(|line| !line.is_empty())
                .collect()
        })
        .filter(|paragraph: &Vec<String>| !paragraph.is_empty())
        .collect()
}

fn main() {
    let mut cmd_options = match CmdOptions::get() {
        Ok(cmd_options) => cmd_options,
        Err(_) => std::process::exit(1),
    };

    let mut contents = Vec::new();
    if let Err(error) = cmd_options.input.read_to_end(&mut contents) {
        eprintln!("Error while reading from input:\n{:?}", error);
        std::process::exit(2);
    };

    let markdown_source = match String::from_utf8(contents) {
        Ok(source) => source,
        Err(_) => {
            eprintln!("Input is not valid UTF-8. This program only supports source files in UTF-8 encoding.");
            std::process::exit(4);
        }
    };

    let markdown_paragraphs = lines_to_paragraphs(markdown_source.lines());

    let parsed_paragraphs = markdown_paragraphs
        .iter()
        .map(|raw_paragraph| match Paragraph::parse(raw_paragraph) {
            Ok(p) => p,
            Err(error) => {
                eprintln!("Failed to parse paragraph: {}", error);
                std::process::exit(5);
            }
        })
        .collect::<Vec<Paragraph>>();

    let mut output = BufWriter::new(cmd_options.output);
    output
        .write(b"<!DOCTYPE html><html><head></head><body>")
        .expect("Failed to write to output");

    for paragraph in parsed_paragraphs {
        output
            .write(paragraph.render_html().as_bytes())
            .expect("Failed to write to output");
    }

    output
        .write(b"</body></html>")
        .expect("Failed to write to output");
}
