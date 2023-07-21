use std::fs;
use std::env;

fn main() -> std::io::Result<()> {
    let current_dir = env::current_dir().unwrap();
    let readme_path = current_dir.join("README.md");
    let readme_template = include_str!("../readme-template.md");
    let readme = readme_template;
    fs::write(readme_path, readme).unwrap();
    Ok(())
}
