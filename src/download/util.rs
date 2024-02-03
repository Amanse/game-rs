use dialoguer::Input;
use eyre::{eyre, Result};
use std::{fs, io::BufWriter};
use tar::Archive;
use xz::read;

pub fn download_and_extract(download_url: &str) -> Result<()> {
    let output = Input::new()
        .with_prompt("Where do you want to download? (put / at the end)")
        .default("".to_string())
        .show_default(false)
        .interact_text()?;

    let file_path = download_to_tmp(download_url, "file")?;

    extract(file_path, output)?;
    Ok(())
}

fn extract(file_path: String, output: String) -> Result<()> {
    let file = fs::File::open(file_path).unwrap();
    println!("Extracting");

    let decomp = read::XzDecoder::new(file);

    let mut a = Archive::new(decomp);

    a.unpack(output).unwrap();

    Ok(())
}

fn download_to_tmp(download_url: &str, name: &str) -> Result<String> {
    let mut file = fs::File::create(format!("{}{}", "/tmp/", name))?;

    let resp = ureq::get(download_url).call()?;

    println!("Downloading, It may look stuck but it is working!");

    if let Err(e) = std::io::copy(&mut resp.into_reader(), &mut BufWriter::new(&mut file)) {
        return Err(eyre!("Could not write to file: {e}"));
    }

    return Ok(format!("/tmp/{}", name));
}
