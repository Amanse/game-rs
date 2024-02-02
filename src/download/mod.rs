pub mod proton;
pub mod ulgwl;

use crate::DownloadOptions;
use eyre::Result;

pub fn download(what: &DownloadOptions) -> Result<()> {
    match what {
        DownloadOptions::Proton => proton::download(),
        DownloadOptions::ULGWL => Ok(()),
    }
}
