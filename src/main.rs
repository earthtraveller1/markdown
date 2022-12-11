use bpaf::Parser;

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

fn main() {
    let cmd_options = get_cmd_options();

    println!("Input: {}", cmd_options.input);
    if let Some(output) = cmd_options.output {
        println!("Output: {}", output);
    }
    if let Some(template) = cmd_options.template {
        println!("Template: {}", template);
    }
}
