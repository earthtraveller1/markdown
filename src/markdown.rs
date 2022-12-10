use std::{fs::File, io::{BufRead, BufReader}};

// A basic file containing the components required for parsing Markdown files.

enum ElementType {
    Paragraph, Header1, Header2, Header3
}

struct Element {
    element_type: ElementType,
    text: String
}

struct Markdown {
    elements: Vec<Element>
}

enum MarkdownError {
    FileOpeningError
}

impl Markdown {
    fn from_file(file_name: &str) -> Result<Markdown, MarkdownError> {
        let file = File::open(file_name);
        if let Ok(file) = file {
            let reader = BufReader::new(file);
            Ok(
                Markdown {
                    elements: reader.lines().map(|line| {
                        let line = line.unwrap();
                        
                        if line.starts_with("# ") {
                            Element {
                                element_type: ElementType::Header1,
                                text: line.strip_prefix("# ").unwrap().to_string()
                            }
                        } else if line.starts_with("## ") {
                            Element {
                                element_type: ElementType::Header2,
                                text: line.strip_prefix("## ").unwrap().to_string()
                            }
                        } else if line.starts_with("### ") {
                            Element {
                                element_type: ElementType::Header3,
                                text: line.strip_prefix("### ").unwrap().to_string()
                            }
                        } else {
                            Element {
                                element_type: ElementType::Paragraph,
                                text: line.clone()
                            }
                        }
                    }).collect()
                }
            )
        } else {
            Err(MarkdownError::FileOpeningError)
        }
    }
}