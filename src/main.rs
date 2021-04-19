use viuer::{print_from_file, Config};
use serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize, Debug)]
struct Neko {
    pub url: String
}

fn main() {
    let conf = Config {
        transparent: true,
        absolute_offset: false,
        x: 0,
        y: 0,
        width: Some(80),
        height: Some(30),
        ..Default::default()
    };

    let req = reqwest::blocking::get("https://nekos.life/api/v2/img/neko").unwrap().text().unwrap();

    let json: Neko = serde_json::from_str(&req).unwrap();

    let img_byte = reqwest::blocking::get(json.url).unwrap().bytes().unwrap();


    if std::path::Path::new("./img.png").exists() {
        std::fs::remove_file("./img.png").unwrap();
    }

    let mut file = std::fs::File::create("./img.png").unwrap();

    std::io::copy(&mut img_byte.as_ref(), &mut file).unwrap();
    print_from_file("./img.png", &conf).expect("Image printing failed.");
}
