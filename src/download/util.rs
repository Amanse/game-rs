use dialoguer::Input;
use eyre::{eyre, Result};
use indicatif::{ProgressBar, ProgressStyle};
use std::{fs, io::BufWriter};
use tar::Archive;

pub fn download_and_extract(download_url: &str, non_ulwgl: bool) -> Result<()> {
    let output = {
        if non_ulwgl {
            Input::new()
                .with_prompt("Where do you want to download? (put / at the end)")
                .default("".to_string())
                .show_default(false)
                .interact_text()?
        } else {
            format!("{}/.local/share/ULWGL", std::env::var("HOME")?)
        }
    };

    download_to_tmp(download_url, "file")?;

    extract("/tmp/file".to_string(), output)?;
    Ok(())
}

fn extract(file_path: String, output: String) -> Result<()> {
    let file = fs::File::open(file_path).unwrap();
    println!("Extracting");

    // @TODO: Make some enum or struct thingy to generalize this
    let decomp = flate2::read::GzDecoder::new(file);
    let mut a = Archive::new(decomp);

    a.unpack(output).unwrap();

    Ok(())
}

fn download_to_tmp(download_url: &str, name: &str) -> Result<String> {
    let mut file = fs::File::create(format!("{}{}", "/tmp/", name))?;

    let resp = ureq::get(download_url).call()?;
    let len = resp
        .header("Content-Length")
        .ok_or(eyre!("Content-Length header not found on request"))?
        .parse()?;

    let pb = ProgressBar::new(len).with_style(ProgressStyle::with_template("{bar:40.green/black} {bytes:>11.green}/{total_bytes:<11.green} {bytes_per_sec:>13.red} eta {eta:.blue}")?);

    if let Err(e) = std::io::copy(
        &mut pb.wrap_read(resp.into_reader()),
        &mut BufWriter::new(&mut file),
    ) {
        return Err(eyre!("Could not write to file: {e}"));
    }

    Ok(format!("/tmp/{}", name))
}
