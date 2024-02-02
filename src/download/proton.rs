use std::{fs::File, io::Write};

use dialoguer::Input;
use eyre::{eyre, Result};
use tar::Archive;
use xz::read;

pub fn download() -> Result<()> {
    let resp: serde_json::Value =
        ureq::get("https://api.github.com/repos/GloriousEggroll/wine-ge-custom/releases/latest")
            .call()?
            .into_json()?;

    let download_url: &str = resp["assets"][1]["browser_download_url"]
        .as_str()
        .ok_or(eyre!("Download link could not be found from github"))?;
    let name: &str = resp["assets"][1]["name"]
        .as_str()
        .ok_or(eyre!("Name of asset could not be detected"))?;

    let input = Input::new()
        .with_prompt("Where do you want to download? (put / at the end)")
        .default("".to_string())
        .show_default(false)
        .interact_text()?;

    let mut file = File::create(format!("{}{}", "/tmp/", name))?;
    let resp = ureq::get(download_url).call()?;
    let len: usize = resp
        .header("Content-Length")
        .ok_or(eyre!("Content-Length header not found on request"))?
        .parse()?;

    println!("Downloading, It may look stuck but it is working!");

    let mut bytes: Vec<u8> = Vec::with_capacity(len);
    resp.into_reader().read_to_end(&mut bytes)?;

    file.write(bytes.as_slice())?;
    let file = File::open(format!("/tmp/{}", name)).unwrap();
    println!("Extracting");

    let decomp = read::XzDecoder::new(file);

    let mut a = Archive::new(decomp);

    a.unpack(input).unwrap();

    Ok(())
}
