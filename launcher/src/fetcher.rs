use std::{fs::File, io::Write, path::Path};

use eyre::Result;

pub struct Fetcher;

impl Fetcher {
    // TODO: Need a actual real API for this
    pub fn get_version() -> Result<String> {
        let resp = attohttpc::get("https://steele.gg/version")
            .header("Authorization", base64::encode("katazina_is_a_jackass_retard"))
            .send()?;

        if !resp.is_success() {
            return Err(eyre::eyre!("Couldn't get version"));
        }

        Ok(resp.text()?)
    }

    // TODO: This should probably get compressed, and this isn't a real API
    /// This will return the path of which the tool was downloaded at.
    pub fn download_tool() -> Result<String> {
        let resp = attohttpc::get("https://steele.gg/tool.exe")
            .header("Authorization", base64::encode("katazina_is_a_jackass_retard"))
            .send()?;

        let path = Path::new("./tool.exe");

        let bytes = resp.bytes()?;

        File::create(&path)?.write_all(&bytes)?;

        Ok(path.to_str().unwrap().into())
    }
}
