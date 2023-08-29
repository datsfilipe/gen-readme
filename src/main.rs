use dialoguer::{console::Term, theme::ColorfulTheme, MultiSelect};
use std::env;
use std::fs;

fn main() {}
#[derive(Debug)]
struct Asset {
    path: String,
    name: String,
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

