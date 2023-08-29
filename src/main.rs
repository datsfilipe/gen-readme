mod args;
mod commands;
mod helpers;

use std::env;

fn main() {
    let args: Vec<String> = env::args().collect();
    if !args::validate_input(&args) {
        return;
    }

    if args.len() == 1 {
        commands::run();
        return;
    }

    match args[1].as_str() {
        "--help" => commands::help::print(),
        "--version" => commands::version::print(),
        "--skip" => commands::skip::run(),
        _ => panic!("Unknown error"),
    }
}
