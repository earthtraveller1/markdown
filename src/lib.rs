use itertools::Itertools;

#[derive(Debug)]
pub struct RawParagraph<'a> {
    lines: Box<[&'a str]>,
}

impl<'a> RawParagraph<'a> {
    pub fn to_line_paragraph(&self) -> LineParagraph<'a> {
        LineParagraph {
            lines: self
                .lines
                .iter()
                .map(|line| {
                    let line_trimmed = line.trim();
                    if line_trimmed.starts_with("#") {
                        Line::Heading {
                            content: line,
                            level: line_trimmed
                                .chars()
                                .take_while(|c| *c == '#')
                                .count()
                                .try_into()
                                .unwrap_or(u8::MAX),
                        }
                    } else if line_trimmed.starts_with("-") {
                        Line::UnorderedListItem {
                            content: line,
                            indents: line
                                .chars()
                                .take_while(|c| c.is_whitespace())
                                .count()
                                .try_into()
                                .unwrap(),
                        }
                    } else if line_trimmed.starts_with(|c: char| c.is_ascii_digit()) {
                        let numbers = line_trimmed.chars().take_while(|c| c.is_ascii_digit());
                        if let Some(expected_dot) = numbers.clone().next() {
                            if expected_dot == '.' {
                                Line::OrderedListItem {
                                    content: line,
                                    indents: line
                                        .chars()
                                        .take_while(|c| c.is_whitespace())
                                        .count()
                                        .try_into()
                                        .unwrap(),
                                    number: numbers.collect::<String>().parse::<u32>().unwrap_or(1),
                                }
                            } else {
                                Line::NormalLine { content: line }
                            }
                        } else {
                            Line::NormalLine { content: line }
                        }
                    } else {
                        Line::NormalLine { content: line }
                    }
                })
                .collect(),
        }
    }
}

pub trait ToRawParagraphs {
    fn to_raw_paragraphs<'a>(&'a self) -> Box<[RawParagraph<'a>]>;
}

impl ToRawParagraphs for str {
    fn to_raw_paragraphs<'a>(&'a self) -> Box<[RawParagraph<'a>]> {
        self.lines()
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
}

#[derive(Debug)]
pub enum Line<'a> {
    NormalLine {
        content: &'a str,
    },
    Heading {
        content: &'a str,
        level: u8,
    },
    UnorderedListItem {
        content: &'a str,
        indents: u8,
    },
    OrderedListItem {
        content: &'a str,
        indents: u8,
        number: u32,
    },
}

#[derive(Debug)]
pub struct LineParagraph<'a> {
    lines: Box<[Line<'a>]>,
}

