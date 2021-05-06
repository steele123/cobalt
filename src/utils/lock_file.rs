use std::{fs, path::Path, time::Duration};

use eyre::{eyre, Result};

pub struct LockFileInfo {
    pub port: i32,
    pub token: String,
}

pub fn parse(lol_path: &str) -> Result<LockFileInfo> {
    let lockfile_path_string = &lol_path.replace("LeagueClient.exe", "lockfile");

    let lockfile_path = Path::new(lockfile_path_string);

    if !lockfile_path.exists() {
        watch_file(lockfile_path)?;
    }

    let contents = fs::read_to_string(&lockfile_path)?;

    let split: Vec<&str> = contents.split(':').collect();

    let lock_file_info = LockFileInfo {
        port: split[2].parse::<i32>().unwrap(),
        token: split[3].into(),
    };

    Ok(lock_file_info)
}

fn watch_file(path: &Path) -> eyre::Result<()> {
    let sw = stopwatch::Stopwatch::start_new();

    let result: eyre::Result<()> = loop {
        if path.exists() {
            break Ok(());
        }

        if sw.elapsed_ms() > 20000 {
            break Err(eyre!("Couldn't find lockfile after 20 seconds..."));
        }

        std::thread::sleep(Duration::from_millis(100));
    };

    result
}
