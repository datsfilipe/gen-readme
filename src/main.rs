use dialoguer::{console::Term, theme::ColorfulTheme, MultiSelect};
use std::env;
use std::fs;

#[derive(Debug, Clone)]
struct Asset {
    path: String,
    name: String,
}

enum Message {
    ValidArguments,
    InvalidArguments,
    TooManyArguments,
    NoArguments,
}

fn main() {
    let args: Vec<String> = env::args().collect();
    if !validate_input(&args) {
        return;
    }

    if args.len() == 1 {
        interactive_mode();
        return;
    }

    match args[1].as_str() {
        "--help" => print_help(),
        "--version" => print_version(),
        "--skip" => skip_interactive_mode(),
        _ => panic!("Unknown error"),
    }
}

fn interactive_mode() {
    let readme_destination_path = env::current_dir().unwrap().join("README.md");
    let assets_dir = "./src/assets";
    let mut assets = get_assets(assets_dir);
    let options = generate_options(&assets);
    let selected = selector(&options);
    assets = filter_assets(&assets, &selected);

    generate_readme(&assets, readme_destination_path.to_str().unwrap());
    println!("README.md generated with selected sections!");
}

fn skip_interactive_mode() {
    let readme_destination_path = env::current_dir().unwrap().join("README.md");
    let assets_dir = "./src/assets";
    let assets = get_assets(assets_dir);
    generate_readme(&assets, readme_destination_path.to_str().unwrap());
    println!("README.md generated with all sections!");
}

fn print_version() {
    println!("readme-generator {}", env!("CARGO_PKG_VERSION"));
}

fn print_help() {
    println!("Usage: readme-generator [options]");
    println!("Options:");
    println!("  --help     Print this help message");
    println!("  --version  Print the version number");
    println!("  --skip     Skip the interactive mode");
}

fn validate_input(args: &Vec<String>) -> bool {
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

fn generate_readme(assets: &Vec<Asset>, destination: &str) {
    let mut readme = String::new();
    let right_order = [
        "title",
        "description",
        "badges",
        "cover",
        "header_end",
        "requirements",
        "installation",
        "configuration",
        "howtouse",
        "howtorun",
        "contributing",
        "contributors",
        "credits",
        "license",
    ];

    for section in right_order {
        if section == "header_end" {
            let header_end = fs::read_to_string("./src/assets/_.md").unwrap();
            readme.push_str(&header_end);
            continue;
        }

        for asset in assets {
            let content = fs::read_to_string(&asset.path).unwrap();
            if section != asset.name {
                continue;
            }
            readme.push_str(&content);
        }
    }

    fs::write(destination, readme).unwrap();
}

fn selector(options: &Vec<&String>) -> Vec<usize> {
    let selection = MultiSelect::with_theme(&ColorfulTheme::default())
        .items(options)
        .interact_on_opt(&Term::stderr())
        .unwrap();

    match selection {
        Some(selection) => return selection,
        None => return vec![],
    }
}

fn get_assets(dir: &str) -> Vec<Asset> {
    let paths = fs::read_dir(dir).unwrap();
    let mut assets: Vec<Asset> = Vec::new();

    for path in paths {
        let path = path.unwrap().path();
        let path_str = path.to_str().unwrap().to_string();
        let name = path
            .file_name()
            .unwrap()
            .to_str()
            .unwrap()
            .to_string()
            .replace(".md", "");
        if name == "_" {
            continue;
        }
        let asset = Asset {
            path: path_str,
            name,
        };
        assets.push(asset);
    }

    return assets;
}

fn filter_assets(assets: &Vec<Asset>, selected: &Vec<usize>) -> Vec<Asset> {
    let mut filtered_assets: Vec<Asset> = Vec::new();

    for i in 0..assets.len() {
        match selected.iter().find(|x| *x == &i) {
            Some(index) => filtered_assets.push(assets[*index].clone()),
            None => continue,
        };
    }

    return filtered_assets;
}

fn generate_options(assets: &Vec<Asset>) -> Vec<&String> {
    let mut options: Vec<&String> = Vec::new();

    for asset in assets {
        options.push(&asset.name);
    }

    return options;
}
