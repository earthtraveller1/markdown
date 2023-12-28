#[derive(Default)]
struct CommandLineOptions {
    input_file: Box<str>,
}

fn main() {
    let options = std::env::args().collect::<Vec<String>>().windows(2).fold(
        CommandLineOptions::default(),
        |mut acc, window| {
            if window.first().unwrap() == "-i" {
                acc.input_file = window.last().unwrap().clone().into_boxed_str();
            }
            return acc;
        },
    );

    if options.input_file.is_empty() {
        eprintln!("[ERROR]: No input file specified.");
        std::process::exit(1);
    }

    let input_content = match std::fs::read_to_string(options.input_file.as_ref()) {
        Ok(x) => x,
        Err(error) => {
            eprintln!("[ERROR]: Failed to load the input file. Error {}", error);
            std::process::exit(1);
        }
    };

    println!("Here is the input file content:\n{}", input_content);
}
