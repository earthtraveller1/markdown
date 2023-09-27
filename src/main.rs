enum Input {
    File(std::fs::File),
    Stdin,
}

enum Output {
    File(std::fs::File),
    Stdout,
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

fn main() {}
