use std::{
    fs::File,
    io::{Read, Write},
    path::{Path, PathBuf},
};

use eyre::Result;
use obfstr::obfstr;
use xz2::read::XzDecoder;

pub struct Fetcher;

impl Fetcher {
    pub fn get_version() -> Result<String> {
        let resp = attohttpc::get(obfstr!("https://api.steele.gg/tools/cobalt/version"))
            .header("Authorization", base64::encode("steele.gg"))
            .send()?;

        if !resp.is_success() {
            return Err(eyre::eyre!("Couldn't get version"));
        }

        Ok(resp.text()?)
    }

    pub fn get_cached_version() -> String {
        let dir = get_version_dir();

        if !Path::new(dir.as_path()).exists() {
            return "0.0.0".into();
        }

        std::fs::read_to_string(dir.to_str().unwrap()).unwrap()
    }

    pub fn save_version(version: String) {
        File::create(get_version_dir().as_path())
            .unwrap()
            .write_all(version.as_bytes())
            .unwrap();
    }

    pub fn get_tool(from_cache: bool) -> Result<String> {
        if from_cache {
            let mut dir = get_cache_dir();

            dir.push(obfstr!("tool.exe"));

            if !Path::new(dir.as_path()).exists() {
                return download_tool();
            }

            return Ok(dir.to_str().unwrap().into());
        }

        download_tool()
    }
}

fn download_tool() -> Result<String> {
    let resp = attohttpc::get(obfstr!("https://api.steele.gg/tools/cobalt.xz"))
        .header("Authorization", base64::encode("steele.gg"))
        .send()?;

    let mut appdata_dir = get_cache_dir();

    appdata_dir.push(obfstr!("tool.exe"));

    let path = Path::new(appdata_dir.as_path());

    let mut file = File::create(&path)?;

    let input = resp.bytes().unwrap();

    let mut bytes: Vec<u8> = Vec::new();

    let mut decomp = XzDecoder::new(input.as_slice());

    decomp.read_to_end(&mut bytes)?;

    file.write_all(bytes.as_slice()).unwrap();

    Ok(path.to_str().unwrap().into())
}

fn get_version_dir() -> PathBuf {
    let mut cached_dir = get_cache_dir();

    cached_dir.push(obfstr!("version"));

    cached_dir
}

fn get_cache_dir() -> PathBuf {
    let mut appdata_dir = dirs::cache_dir().unwrap();

    appdata_dir.push(obfstr!("Cobalt"));

    if !Path::new(appdata_dir.as_path()).exists() {
        std::fs::create_dir(appdata_dir.clone()).unwrap();
    }

    appdata_dir
}
