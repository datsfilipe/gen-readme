pub mod help;
pub mod skip;
pub mod version;

use crate::helpers;
use dialoguer::{console::Term, theme::ColorfulTheme, MultiSelect};
use std::env;

pub fn run() {
    let readme_destination_path = env::current_dir().unwrap().join("README.md");
    let mut assets = helpers::get_assets();
    let options = helpers::generate_options(&assets);
    let selected = selector(&options);
    assets = helpers::filter_assets(&assets, &selected);

    helpers::generate_readme(&assets, readme_destination_path.to_str().unwrap());
    println!("README.md generated with selected sections!");
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
