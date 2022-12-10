mod markdown;

use markdown::Markdown;

fn main() {
    let markdown = Markdown::from_file("misc/test_file.md").unwrap();
    
    println!("{}", markdown.to_html());
}
