use std::fs;

#[derive(Debug, Clone)]
pub struct Asset {
    content: String,
    name: String,
}

const ORDER: [&str; 14] = [
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

pub fn generate_readme(assets: &Vec<Asset>, destination: &str) {
    let mut readme = String::new();

    for section in ORDER {
        for asset in assets {
            let content = &asset.content;
            if section != asset.name {
                continue;
            }
            readme.push_str(&content);
        }
    }

    fs::write(destination, readme).unwrap();
}

pub fn get_assets() -> Vec<Asset> {
    let mut assets: Vec<Asset> = Vec::new();
    let mut contents: Vec<String> = Vec::new();

    // TODO: Find a better way to do this (don't know if it's possible :P)
    contents.push(include_str!("assets/title.md").to_string());
    contents.push(include_str!("assets/description.md").to_string());
    contents.push(include_str!("assets/badges.md").to_string());
    contents.push(include_str!("assets/cover.md").to_string());
    contents.push(include_str!("assets/_.md").to_string());
    contents.push(include_str!("assets/requirements.md").to_string());
    contents.push(include_str!("assets/installation.md").to_string());
    contents.push(include_str!("assets/configuration.md").to_string());
    contents.push(include_str!("assets/howtouse.md").to_string());
    contents.push(include_str!("assets/howtorun.md").to_string());
    contents.push(include_str!("assets/contributing.md").to_string());
    contents.push(include_str!("assets/contributors.md").to_string());
    contents.push(include_str!("assets/credits.md").to_string());
    contents.push(include_str!("assets/license.md").to_string());

    for i in 0..contents.len() {
        let name = ORDER[i].to_string();
        let content = contents[i].clone();
        let asset = Asset { name, content };
        assets.push(asset);
    }

    return assets;
}

pub fn filter_assets(assets: &Vec<Asset>, selected: &Vec<usize>) -> Vec<Asset> {
    let mut filtered_assets: Vec<Asset> = Vec::new();

    for i in 0..assets.len() {
        match selected.iter().find(|x| *x == &i) {
            Some(index) => filtered_assets.push(assets[*index].clone()),
            None => continue,
        };
    }

    return filtered_assets;
}

pub fn generate_options(assets: &Vec<Asset>) -> Vec<&String> {
    let mut options: Vec<&String> = Vec::new();

    for asset in assets {
        options.push(&asset.name);
    }

    return options;
}
