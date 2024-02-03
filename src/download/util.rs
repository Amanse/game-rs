use dialoguer::Input;
use eyre::{eyre, Result};
use std::{fs, io::BufWriter};
use tar::Archive;

pub fn download_and_extract(download_url: &str, is_xz: bool) -> Result<()> {
    let output = Input::new()
        .with_prompt("Where do you want to download? (put / at the end)")
        .default("".to_string())
        .show_default(false)
        .interact_text()?;

    let file_path = download_to_tmp(download_url, "file")?;

    extract(file_path, output, is_xz)?;
    Ok(())
}

fn extract(file_path: String, output: String, is_xz: bool) -> Result<()> {
    let file = fs::File::open(file_path).unwrap();
    println!("Extracting");

    // @TODO: Make some enum or struct thingy to generalize this
    if is_xz {
        let decomp = xz::read::XzDecoder::new(file);
        let mut a = Archive::new(decomp);

        a.unpack(output).unwrap();
    } else {
        let decomp = flate2::read::GzDecoder::new(file);
        let mut a = Archive::new(decomp);

        a.unpack(output).unwrap();
    }

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
