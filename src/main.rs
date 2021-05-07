#![warn(clippy::all, clippy::pedantic, clippy::nursery, clippy::perf)]
#![allow(
    clippy::cast_possible_truncation,
    clippy::module_name_repetitions,
    non_snake_case,
    dead_code,
    clippy::cast_possible_wrap,
    clippy::upper_case_acronyms
)]

use std::sync::{Arc, Mutex};

use process_worker::Events;
use utils::{
    input::{Key, KeyListener, Modifiers},
    lcu::Endpoints,
    process::league_exists,
};

use crate::utils::lcu::Method;

mod utils;

mod process_worker;

macro_rules! enclose {
    ( ($( $x:ident ),*) $y:expr ) => {
        {
            $(let $x = $x.clone();)*
            $y
        }
    };
}

fn main() -> eyre::Result<()> {
    println!("Trying to find the LeagueClient.exe process...");

    let sw = stopwatch::Stopwatch::start_new();

    loop {
        if league_exists() {
            println!("Found LeagueClient.exe in {}ms!", sw.elapsed_ms());
            break;
        }

        std::thread::sleep(std::time::Duration::from_secs(2));
    }

    let path = utils::process::get_lock_file_path().unwrap();

    let lock_file_info = utils::lock_file::parse(&path).unwrap();

    let lcu = Arc::new(Mutex::new(
        utils::lcu::LCUClient::new(&lock_file_info.token, lock_file_info.port).unwrap(),
    ));

    let rx = process_worker::spawn();

    println!("Controls\nCTRL+D to dodge your current champ select.\nCTRL+B to aram boost");

    std::thread::spawn(enclose! { (lcu) move || {
        let mut key_listener = KeyListener::new();

        key_listener
            .register_hotkey(
                Modifiers::CTRL,
                Key::D,
                enclose! {(lcu) move || {
                    println!("Lobby Crash Queued...");
                    #[cfg(not(debug_assertions))]
                    lcu.lock().unwrap().crash_lobby().unwrap();
                    #[cfg(debug_assertions)]
                    println!("Debug Assertions are on so you don't go into TFT");

                println!("Lobby has been dodged, you can leave the TFT game ~45 seconds.");
                }},
            )
            .unwrap();

        key_listener
            .register_hotkey(
                Modifiers::CTRL,
                Key::B,
                enclose! {(lcu) move || {
                    println!("ARAM Boost Queued...");
                    lcu
                        .lock()
                        .unwrap()
                        .send(&Endpoints::AramBoost, &Method::POST, "")
                        .unwrap();
                    println!("ARAM Boost Completed...");
                }},
            )
            .unwrap();

        key_listener.listen();
    }});

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
