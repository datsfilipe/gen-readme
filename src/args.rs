enum Message {
    ValidArguments,
    InvalidArguments,
    TooManyArguments,
    NoArguments,
}

pub fn validate_input(args: &Vec<String>) -> bool {
    let message = verify_args(args);
    match message {
        Message::ValidArguments => return true,
        Message::NoArguments => return true,
        Message::InvalidArguments => return false,
        Message::TooManyArguments => return false,
    }
}

fn verify_args(args: &Vec<String>) -> Message {
    match args.len() {
        1 => {
            println!("Skipping interactive mode, use --help for more information");
            return Message::NoArguments;
        }
        2 => {
            if args[1] == "--help" {
                return Message::ValidArguments;
            }
            if args[1] == "--version" {
                return Message::ValidArguments;
            }
            if args[1] == "--skip" {
                return Message::ValidArguments;
            }
            println!("Invalid arguments provided, use --help for more information");
            return Message::InvalidArguments;
        }
        _ => {
            println!("Too many arguments provided, use --help for more information");
            return Message::TooManyArguments;
        }
    }
}
