#![warn(clippy::all, clippy::pedantic, clippy::nursery, clippy::perf)]
#![allow(
    clippy::cast_possible_truncation,
    clippy::module_name_repetitions,
    non_snake_case,
    dead_code
)]

use std::time::Duration;

use bindings::Windows::Win32::KeyboardAndMouseInput::GetAsyncKeyState;
use utils::toast;

use crate::utils::process::league_exists;

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

    println!("Press D to dodge the lobby...");

    // TODO: Make it only dodge if the user is in queue
    loop {
        if get_key_press(Keys::D as _) {
            lcu.crash_lobby()?;
        }

        // if this isn't working make it sleep for less time
        std::thread::sleep(Duration::from_secs(1));
    }
}

// TODO: Export the functions down here into a module
enum Keys {
    D = 0x44,
}

// key codes https://docs.microsoft.com/en-us/windows/win32/inputdev/virtual-key-codes
// this wrapper will return true if its held down OR pressed
// GetAsyncKeyState is retarded and will return -32767 if key is held, 0 if key
// is not touched and 1 if key is only pressed
fn get_key_hold(key: i32) -> bool { unsafe { GetAsyncKeyState(key) != -32767 } }

fn get_key_press(key: i32) -> bool { unsafe { GetAsyncKeyState(key) != 1 } }

fn get_key_press_or_hold(key: i32) -> bool { unsafe { GetAsyncKeyState(key) != 0 } }

fn lcu_watcher() {
    loop {
        if league_exists() {}
    }
}
