use std::io::{Read, Write};

enum Input {
    File(std::fs::File),
    Stdin,
}

impl Read for Input {
    fn read(&mut self, buf: &mut [u8]) -> Result<usize, std::io::Error> {
        match self {
            Input::File(file) => file.read(buf),
            Input::Stdin => std::io::stdin().read(buf),
        }
    }
}

enum Output {
    File(std::fs::File),
    Stdout,
}

impl Write for Output {
    fn write(&mut self, buf: &[u8]) -> std::io::Result<usize> {
        match self {
            Output::File(file) => file.write(buf),
            Output::Stdout => std::io::stdout().write(buf),
        }
    }

    fn flush(&mut self) -> std::io::Result<()> {
        match self {
            Output::File(file) => file.flush(),
            Output::Stdout => std::io::stdout().flush(),
        }
    }
}

struct CmdOptions {
    input: Input,
    output: Output,
}

impl CmdOptions {
    fn get() -> Result<CmdOptions, ()> {
        let mut cmd_args = std::env::args();
        let mut cmd_options = CmdOptions {
            input: Input::Stdin,
            output: Output::Stdout,
        };

        // The first command line argument would be the program, so we have to first advance onto
        // the second one.
        cmd_args.next();

        while let Some(arg) = cmd_args.next() {
            if arg == "-o" || arg == "--output" {
                match cmd_args.next() {
                    Some(path) => {
                        cmd_options.output = Output::File({
                            let file = std::fs::File::create(path.as_str());
                            match file {
                                Ok(file) => file,
                                Err(_) => {
                                    eprintln!("[ERROR]: Could not open '{}' for writing.", path);
                                    return Err(());
                                }
                            }
                        });
                    }
                    None => {
                        eprintln!("[ERROR]: Expected filename after '{}'", arg);
                        return Err(());
                    }
                }
            } else {
                cmd_options.input = {
                    match std::fs::File::open(arg.as_str()) {
                        Ok(file) => Input::File(file),
                        Err(_) => {
                            eprintln!("[ERROR]: Could not open '{}'", arg);
                            return Err(());
                        }
                    }
                }
            }
        }

        Ok(cmd_options)
    }
}

fn main() {
    let mut cmd_options = match CmdOptions::get() {
        Ok(cmd_options) => cmd_options,
        Err(_) => std::process::exit(1),
    };

    let mut contents = Vec::new();
    if let Err(error) = cmd_options.input.read_to_end(&mut contents) {
        eprintln!("Error while reading from input:\n{:?}", error);
        std::process::exit(2);
    };

    if let Err(error) = cmd_options.output.write(&contents) {
        eprintln!("Error while writing to output:\n{:?}", error);
        std::process::exit(3);
    }
}
