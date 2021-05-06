#![warn(clippy::all, clippy::pedantic, clippy::nursery, clippy::perf)]
#![allow(
    clippy::cast_possible_truncation,
    clippy::module_name_repetitions,
    non_snake_case,
    dead_code,
    clippy::cast_possible_wrap,
    clippy::upper_case_acronyms
)]

use utils::{
    input::{Key, KeyListener, Modifiers},
    lcu::Endpoints,
    process::league_exists,
    toast,
};

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

    println!("Controls\nCTRL+D to dodge your current champ select.\nCTRL+B to aram boost");

    let mut key_listener = KeyListener::new();

    // TODO: Make it only dodge if the user is in champ select

    key_listener
        .register_hotkey(Modifiers::CTRL, Key::D, move || {
            println!("Lobby Crash Queued...");
            #[cfg(not(debug_assertions))]
            lcu.crash_lobby().unwrap();
            #[cfg(debug_assertions)]
            println!("Debug Assertions are on so you don't go into TFT");
        })
        .unwrap();

    key_listener
        .register_hotkey(Modifiers::CTRL, Key::B, move || {
            println!("ARAM Boost Queued...");
            lcu.send(&Endpoints::AramBoost, &Method::POST, "").unwrap();
            println!("ARAM Boost Completed...");
        })
        .unwrap();

    // VERY SYNC, JUST PARSES ALL OF THE MESSAGES
    key_listener.listen();

    Ok(())
}

fn lcu_watcher() {
    loop {
        if league_exists() {}
    }
}
