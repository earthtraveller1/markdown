mod markdown;

use markdown::Markdown;

struct CmdOptions {
    input: Option<String>,
    output: String,
    template: Option<String>
}

fn get_cmd_options() -> CmdOptions {
    let mut options = CmdOptions {
        input: None,
        output: "index.html".to_string(),
        template: None
    };
    
    // This states what the next argument will be.
    #[derive(Copy, Clone)]
    enum NextArgument {
        Input, Output, Template
    }
    
    let mut next_arg = NextArgument::Input;
    
    let mut args = std::env::args().collect::<Vec<String>>();
    args.remove(0); // The first argument is the program name.
    
    args.iter().for_each(|argument| {
        match argument.as_str() {
            "-o" => next_arg = NextArgument::Output,
            "-t" => next_arg = NextArgument::Template,
            "--output" => next_arg = NextArgument::Output,
            "--template" => next_arg = NextArgument::Template,
            _ => match next_arg {
                NextArgument::Input => options.input = Some(argument.clone()),
                NextArgument::Output => {
                    options.output = argument.clone();
                    next_arg = NextArgument::Input;
                },
                NextArgument::Template => {
                    options.template = Some(argument.clone());
                    next_arg = NextArgument::Input;
                }
            }
        }
    });
    
    options
}

fn main() {
    let markdown = Markdown::from_file("misc/test_file.md").unwrap();
    let options = get_cmd_options();
    
    println!("{}", markdown.to_html());
    
    if options.input.is_none() {
        println!("Please specify an input file.");
        return;
    }
    
    let input = options.input.unwrap();
    
    println!("The input file: {}", input);
    println!("The output file: {}", options.output);
    
    if let Some(template) = options.template {
        println!("The template file: {}", template);
    }
}
