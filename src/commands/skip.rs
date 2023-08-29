use crate::helpers;
use std::env;

pub fn run() {
    let readme_destination_path = env::current_dir().unwrap().join("README.md");
    let assets_dir = "./src/assets";
    let assets = helpers::get_assets(assets_dir);
    helpers::generate_readme(&assets, readme_destination_path.to_str().unwrap());
    println!("README.md generated with all sections!");
}
