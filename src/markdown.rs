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