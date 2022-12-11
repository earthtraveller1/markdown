use bpaf::Parser;
use std::fs::File;
use std::io::Write;

const DEFAULT_TEMPLATE: &str = r#"<!DOCTYPE html>
<html>
<head><title>Parsed Markdown Document</title>
</head>
<body>
$$$body$$$
</body>
</html>
"#;

struct CmdOptions {
    output: Option<String>,
    template: Option<String>,
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
    let input = bpaf::positional::<String>("INPUT").help("The Markdown file to parse.");

    bpaf::construct!(CmdOptions {
        output,
        template,
        input
    })
    .to_options()
    .descr("This is a description")
    .run()
}

fn load_and_parse_markdown(file_name: &str) -> String {
    let markdown_content = std::fs::read_to_string(file_name).unwrap_or_else(|_| {
        eprintln!("Could not load or access {}.", file_name);
        std::process::exit(-1);
    });
    
    markdown::to_html(markdown_content.as_str())
}

fn main() {
    let cmd_options = get_cmd_options();
    
    let template = if let Some(template) = cmd_options.template {
        std::fs::read_to_string(template.as_str()).unwrap_or_else(|_| {
            eprintln!("Could not load or access {}.", template.as_str());
            std::process::exit(-1);
        })
    } else { DEFAULT_TEMPLATE.to_string() };
    
    let content = load_and_parse_markdown(cmd_options.input.as_str());
    
    let output = template.replace("$$$body$$$", content.as_str());
    
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
