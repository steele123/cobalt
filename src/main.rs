#![warn(clippy::all, clippy::pedantic, clippy::nursery, clippy::perf)]

use reqwest::Response;

use crate::utils::lcu::{Endpoints, LCUClient};

mod utils;

#[tokio::main]
async fn main() -> eyre::Result<()> {
    let path = utils::process::get_lock_file_path().unwrap();

    // get lock
    let lock_file_info = utils::lock_file::parse(&path).unwrap();

    let lcu = utils::lcu::LCUClient::new(&lock_file_info.token, lock_file_info.port).unwrap();

    lcu.crash_lobby();

    Ok(())
}
