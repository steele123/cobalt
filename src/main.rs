#![warn(clippy::all, clippy::pedantic, clippy::nursery, clippy::perf)]
#![allow(
    clippy::cast_possible_truncation,
    clippy::module_name_repetitions,
    non_snake_case,
    dead_code
)]

use std::time::Duration;

use utils::{input::*, lcu::Endpoints, process::league_exists, toast};

use crate::utils::lcu::Method;

mod utils;

fn main() -> eyre::Result<()> {
    toast::send("Trying to find the LeagueClient.exe process...")?;

    let sw = stopwatch::Stopwatch::start_new();

    loop {
        if league_exists() {
            toast::send(&format!("Found LeagueClient.exe in {}ms!", sw.elapsed_ms()))?;
            break;
        }

        // slow down loop a bit
        std::thread::sleep(std::time::Duration::from_secs(2));
    }

    let path = utils::process::get_lock_file_path().unwrap();

    let lock_file_info = utils::lock_file::parse(&path).unwrap();

    let lcu = utils::lcu::LCUClient::new(&lock_file_info.token, lock_file_info.port).unwrap();

    // TODO: Need a thread to check if the league client is open.

    println!("CONTROLS\nCTRL+D to dodge your current champ select.\nCTRL+B to aram boost");

    // TODO: Make it only dodge if the user is in champ select
    loop {
        if get_key_press_or_hold(Key::CONTROL) {
            if get_key_press(Key::D) {
                println!("Pressed Ctrl+D");
                // don't want to keep fucking going into a tft
                #[cfg(not(debug_assertions))]
                lcu.crash_lobby()?;
            }

            if get_key_press(Key::B) {
                println!("Pressed Ctrl+B");
                lcu.send(&Endpoints::AramBoost, &Method::POST, "")?;
            }
        }

        // if this isn't working make it sleep for less time
        std::thread::sleep(Duration::from_millis(500));
    }
}

fn lcu_watcher() {
    loop {
        if league_exists() {}
    }
}
