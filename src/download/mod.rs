mod util;

use crate::DownloadOptions;
use eyre::{eyre, Result};

pub fn download(what: &DownloadOptions) -> Result<()> {
    match what {
        DownloadOptions::Proton => util::download_and_extract(&get_proton_url()?, true),
        DownloadOptions::ULGWL => util::download_and_extract(
            "https://api.github.com/repos/Open-Wine-Components/ULWGL-launcher/tarball",
            false,
        ),
    }
}

fn get_proton_url() -> Result<String> {
    let resp: serde_json::Value =
        ureq::get("https://api.github.com/repos/GloriousEggroll/wine-ge-custom/releases/latest")
            .call()?
            .into_json()?;

    Ok(resp["assets"][1]["browser_download_url"]
        .as_str()
        .ok_or(eyre!("Could not get download link from github"))?
        .to_string())
}
