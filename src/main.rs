use dialoguer::{console::Term, theme::ColorfulTheme, MultiSelect};
use std::env;
use std::fs;

#[derive(Debug, Clone)]
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
