use bpaf::Parser;
use std::fs::File;
use std::io::Write;

const DEFAULT_TEMPLATE: &str = r#"<!DOCTYPE html>
<!DOCTYPE html>
<html>
    <head>
        <title>$$$title$$$</title>
        <style>
            body {
                font-family: 'Times New Roman';
                font-size: 1.25em;
                margin: auto;
                max-width: 48em;
            }
            
            h1 {
                font-size: 3em;
            }
            
            h2 {
                font-size: 2em;
            }
            
            h3 {
                font-size: 1.5em;
            }
            
            p {
                text-indent: 2.5em;
            }
        </style>
    </head>
    <body>
        $$$body$$$
    </body>
</html>
"#;

struct CmdOptions {
    output: Option<String>,
    template: Option<String>,
    scuff: bool,
    input: String,
}

fn get_cmd_options() -> CmdOptions {
    let output = bpaf::short('o')
        .help("The file that the resulting HTML should be outputted to.")
        .long("output")
        .argument::<String>("OUTPUT")
        .optional();
    let template = bpaf::short('t')
        .help("The HTML file to use as the template.")
        .long("template")
        .argument::<String>("TEMPLATE")
        .optional();
    let scuff = bpaf::short('s')
        .help("Scuff the output file to reduce file size.")
        .long("scuff")
        .switch();
    let input = bpaf::positional::<String>("INPUT").help("The Markdown file to parse.");

    bpaf::construct!(CmdOptions {
        output,
        template,
        scuff,
        input
    })
    .to_options()
    .descr("A simple CLI program for converting a Markdown file into an HTML file.")
    .run()
}

struct Markdown {
    title: String,
    html: String
}

fn load_and_parse_markdown(file_name: &str) -> Markdown {
    let markdown_content = std::fs::read_to_string(file_name).unwrap_or_else(|_| {
        eprintln!("Could not load or access {}.", file_name);
        std::process::exit(-1);
    });
    
    let title = if let Some(title) = markdown_content.lines().find(|line| line.starts_with("# ")) {
        let mut title = title.to_string();
        
        // Remove the # character and the space.
        title.remove(0);
        title.remove(0);
        
        title
    } else {
        "Markdown Document".to_string()
    };
    
    Markdown {
        title,
        html: markdown::to_html(markdown_content.as_str())
    }
}

fn scuff(string: &str) -> String {
    let words = string.split_whitespace().collect::<Vec<&str>>();
    
    let mut string = String::new();
    words.iter().for_each(|word| {string.push_str(word); string.push(' ');});
    
    string
}

fn main() {
    let cmd_options = get_cmd_options();
    
    let template = if let Some(template) = cmd_options.template {
        std::fs::read_to_string(template.as_str()).unwrap_or_else(|_| {
            eprintln!("Could not load or access {}.", template.as_str());
            std::process::exit(-1);
        })
    } else { DEFAULT_TEMPLATE.to_string() };
    
    let markdown = load_and_parse_markdown(cmd_options.input.as_str());
    
    let mut output = template.replace("$$$body$$$", markdown.html.as_str());
    output = output.replace("$$$title$$$", markdown.title.as_str());
    
    if cmd_options.scuff {
        output = scuff(output.as_str());
    }
    
    let output_file = if let Some(output) = cmd_options.output {
        output
    } else {
        "index.html".to_string()
    };
    
    let mut output_file = File::create(output_file.as_str()).unwrap_or_else(|_| {
        eprintln!("Cannot create or open {} for writing.", output_file);
        std::process::exit(-1);
    });
    
    let output = output.into_bytes();
    output_file.write(&output[..]).unwrap_or_else(|_| {
        eprintln!("Was unable to write output to file.");
        std::process::exit(-1);
    });
}
