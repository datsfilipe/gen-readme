use std::fs;

#[derive(Debug, Clone)]
pub struct Asset {
    path: String,
    name: String,
}

pub fn generate_readme(assets: &Vec<Asset>, destination: &str) {
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

pub fn get_assets(dir: &str) -> Vec<Asset> {
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
