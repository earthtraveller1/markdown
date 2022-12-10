// A basic file containing the components required for parsing Markdown files.

enum ElementType {
    Paragraph, Header1, Header2, Header3
}

struct Element {
    element_type: ElementType,
    text: String
}

pub struct Markdown {
    elements: Vec<Element>
}

#[derive(Debug)]
pub enum MarkdownError {
    FileReadError
}

impl Markdown {
    pub fn from_file(file_name: &str) -> Result<Markdown, MarkdownError> {
        let contents = std::fs::read_to_string(file_name);
        if let Ok(contents) = contents {
            Ok(Markdown::from_str(contents.as_str()))
        } else {
            Err(MarkdownError::FileReadError)
        }
    }
    
    pub fn from_str(contents: &str) -> Markdown {
        Markdown {
            elements: contents.lines().map(|line| {
                let line = line.trim();
                
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
                        text: line.to_string()
                    }
                }
            }).collect()
        }
    }
    
    pub fn to_html(&self) -> String {
        let mut result = String::new();
        
        self.elements.iter().for_each(|element| {
            match element.element_type {
                ElementType::Paragraph => {
                    result.push_str("<p>");
                    result.push_str(element.text.as_str());
                    result.push_str("</p>");
                },
                ElementType::Header1 => {
                    result.push_str("<h1>");
                    result.push_str(element.text.as_str());
                    result.push_str("</h1>");
                }
                ElementType::Header2 => {
                    result.push_str("<h2>");
                    result.push_str(element.text.as_str());
                    result.push_str("</h2>");
                }
                ElementType::Header3 => {
                    result.push_str("<h3>");
                    result.push_str(element.text.as_str());
                    result.push_str("</h3>");
                }
            }
        });
        
        result
    }
}