mod util;

use crate::DownloadOptions;
use eyre::{eyre, Result};

pub fn download(what: &DownloadOptions) -> Result<()> {
    match what {
        DownloadOptions::Proton => util::download_and_extract(&get_proton_url()?, true),
        DownloadOptions::ULWGL => util::download_and_extract(&get_ulwgl_url()?, false),
    }
}

fn get_ulwgl_url() -> Result<String> {
    get_latest_release(
        "https://api.github.com/repos/Open-Wine-Components/ULWGL-launcher".to_string(),
    )
}

fn get_proton_url() -> Result<String> {
    get_latest_release("https://api.github.com/repos/GloriousEggroll/proton-ge-custom".to_string())
}

fn get_latest_release(repo: String) -> Result<String> {
    let resp: serde_json::Value = ureq::get(&format!("{}/releases/latest", repo))
        .call()?
        .into_json()?;

    Ok(resp["assets"][1]["browser_download_url"]
        .as_str()
        .ok_or(eyre!("Could not get download link from github"))?
        .to_string())
}
