use std::{fs::File, io::Write};

use dialoguer::Input;

pub fn download() {
    let resp: serde_json::Value = ureq::get("https://api.github.com/repos/GloriousEggroll/wine-ge-custom/releases/latest")
        .call().unwrap()
        .into_json().unwrap();

    let download_url:&str = resp["assets"][1]["browser_download_url"].as_str().unwrap();
    let name: &str = resp["assets"][1]["name"].as_str().unwrap();

    let input = Input::new()
        .with_prompt("Where do you want to download? (put / at the end)")
        .default("".to_string())
        .show_default(false)
        .interact_text()
        .unwrap();

    let mut file = File::create(format!("{}{}", input, name)).unwrap();
    let resp = ureq::get(download_url).call().unwrap();
    let len: usize = resp.header("Content-Length")
    .unwrap()
    .parse().unwrap();

    println!("Downloading, It may look stuck but it is working!");

    let mut bytes: Vec<u8> = Vec::with_capacity(len);
    resp.into_reader()
        .read_to_end(&mut bytes).unwrap();

    file.write(bytes.as_slice()).unwrap();
}
