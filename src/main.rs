#![warn(clippy::all, clippy::pedantic, clippy::nursery, clippy::perf)]
#![allow(
    clippy::cast_possible_truncation,
    clippy::module_name_repetitions,
    non_snake_case,
    dead_code
)]

use std::time::Duration;

use process_worker::Events;
use utils::{
    input::{get_key_press, get_key_press_or_hold, Key},
    lcu::{Endpoints, Method},
    process::league_exists,
    toast,
};

mod utils;

mod process_worker;

fn main() -> eyre::Result<()> {
    toast::send("Trying to find the LeagueClient.exe process...")?;

    let sw = stopwatch::Stopwatch::start_new();

    loop {
        if league_exists() {
            toast::send(&format!("Found LeagueClient.exe in {}ms!", sw.elapsed_ms()))?;
            break;
        }

        std::thread::sleep(std::time::Duration::from_secs(2));
    }

    let path = utils::process::get_lock_file_path().unwrap();

    let lock_file_info = utils::lock_file::parse(&path).unwrap();

    let lcu = std::sync::Arc::new(std::sync::Mutex::new(
        utils::lcu::LCUClient::new(&lock_file_info.token, lock_file_info.port).unwrap(),
    ));

    let lcu_clone = std::sync::Arc::clone(&lcu);

    let rx = process_worker::spawn();

    println!("Controls\n>CTRL+D to dodge your current champ select.\n>CTRL+B to aram boost");

    // TODO: Make it only dodge if the user is in champ select
    std::thread::spawn(move || {
        loop {
            // TODO: JSON config for keys (maybe)

            if !lcu_clone.lock().unwrap().can_send {
                std::thread::sleep(Duration::from_millis(200));
                continue;
            }

            if get_key_press_or_hold(Key::CONTROL) {
                if get_key_press(Key::D) {
                    println!("Lobby Crash Queued...");
                    // don't want to keep fucking going into a tft
                    #[cfg(not(debug_assertions))]
                    lcu_clone.crash_lobby().unwrap();
                    println!("Completed Lobby Crash");
                }

                if get_key_press(Key::B) {
                    println!("ARAM Boost Queued...");
                    lcu_clone
                        .lock()
                        .unwrap()
                        .send(&Endpoints::AramBoost, &Method::POST, "")
                        .unwrap();
                    println!("Completed ARAM Boost");
                }
            }

            std::thread::sleep(Duration::from_millis(200));
        }
    });

    while let Ok(event) = rx.recv() {
        match event {
            Events::Connected => {
                let path = utils::process::get_lock_file_path().unwrap();
                let lock_file_info = utils::lock_file::parse(&path).unwrap();
                lcu.lock().unwrap().reconnect(&lock_file_info.token, lock_file_info.port);
            },
            Events::Disconnected => {
                lcu.lock().unwrap().disconnect();
            },
        }
    }

    Ok(())
}
