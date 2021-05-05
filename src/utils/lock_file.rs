use std::fs;

use eyre::Result;

pub struct LockFileInfo {
    pub port: i32,
    pub token: String,
}

// TODO: Need to wait for the lockfile to be created because the lockfile isn't
// always there if league is open
pub fn parse(lol_path: &str) -> Result<LockFileInfo> {
    let lockfile_path = lol_path.replace("LeagueClient.exe", "lockfile");

    if fs::metadata(&lockfile_path).is_err() {
        panic!("File doesn't exist");
    }

    let contents = fs::read_to_string(&lockfile_path)?;

    let split: Vec<&str> = contents.split(':').collect();

    let lock_file_info = LockFileInfo {
        port: split[2].parse::<i32>().unwrap(),
        token: split[3].into(),
    };

    Ok(lock_file_info)
}
