use std::{
    fs::File,
    io::{Read, Seek, SeekFrom, Write},
    path::{Path, PathBuf},
};

use eyre::Result;
use rand::{distributions::Alphanumeric, thread_rng, Rng};
use tempfile::NamedTempFile;

use crate::process;

pub struct Fetcher;

impl Fetcher {
    pub fn launch_tool() -> Result<()> {
        let resp = attohttpc::get("https://github.com/steele123/cobalt/releases/latest/download/cobalt.exe").send()?;

        let mut tempfile = NamedTempFile::new().unwrap();

        let bytes = resp.bytes().unwrap();

        tempfile.write_all(bytes.as_slice()).unwrap();

        tempfile.seek(SeekFrom::Start(0)).unwrap();

        tempfile.close();

        process::create_process(tempfile.path().to_str().unwrap().into())
            .expect("Couldn't create a process from that path!");

        Ok(())
    }
}

fn rand(size: usize) -> String {
    std::iter::repeat(())
        .map(|()| thread_rng().sample(Alphanumeric))
        .map(char::from)
        .take(10)
        .collect()
}
