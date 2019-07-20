extern crate reqwest;

use std::fs::File;
use std::path::Path;

fn get(path: &str, url: &str) {
    println!("Fetching {}", url);
    let mut body = reqwest::get(url)
        .expect(&format!("Failed to fetch {}", url));
    let full_path = Path::new("testdata").join(path);
    let mut file = File::create(&full_path).expect(&format!("Failed to create {:?}", &full_path));
    body.copy_to(&mut file).expect(&format!("Failed to write {:?}", &full_path));
}

fn main() {
    get("curseforge/omnifactory.html", "https://www.curseforge.com/minecraft/modpacks/omnifactory");
    get("curseforge/omnifactory_file.html", "https://www.curseforge.com/minecraft/modpacks/omnifactory/files/2733453");
    get("curseforge/omnifactory.zip", "https://www.curseforge.com/minecraft/modpacks/omnifactory/download/2733453/file");
}
