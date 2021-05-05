#![warn(clippy::all, clippy::pedantic, clippy::nursery, clippy::perf)]

use reqwest::Response;

use crate::utils::{
    lcu::{Endpoints, LCUClient},
    process::league_exists,
};

mod utils;

#[tokio::main]
async fn main() -> eyre::Result<()> {
    println!("Trying to find the LeagueClient.exe process...");

    let sw = stopwatch::Stopwatch::start_new();

    loop {
        if league_exists() {
            println!("Found LeagueClient.exe in {}ms!", sw.elapsed_ms());
            break;
        }

        // slow down loop a bit
        tokio::time::sleep(tokio::time::Duration::from_secs(2));
    }

    let path = utils::process::get_lock_file_path().unwrap();

    let lock_file_info = utils::lock_file::parse(&path).unwrap();

    let lcu = utils::lcu::LCUClient::new(&lock_file_info.token, lock_file_info.port).unwrap();

    lcu.crash_lobby().await;

    println!("Champ Select lobby was crashed successfully");

    Ok(())
}
