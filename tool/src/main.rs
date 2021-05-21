#![feature(in_band_lifetimes)]
#![warn(clippy::all, clippy::pedantic, clippy::nursery, clippy::perf)]
#![allow(
    clippy::cast_possible_truncation,
    clippy::module_name_repetitions,
    non_snake_case,
    dead_code,
    clippy::cast_possible_wrap,
    clippy::upper_case_acronyms
)]

use colored::*;
use console::Console;
use iced::{Application, Settings};
use image::GenericImageView;
use process_worker::Events;
use utils::{
    input::{Key, KeyListener, Modifiers},
    lcu::{Endpoints, Method},
    process::league_exists,
};

mod utils;

mod console;

mod app;

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
    let bytes = include_bytes!("../../shared/resources/icon.ico");
    let img = image::load_from_memory(bytes).unwrap();
    let img_dims = img.dimensions();
    let img_raw = img.into_rgb8().into_raw();
    let icon = iced::window::Icon::from_rgba(img_raw, img_dims.0, img_dims.1).unwrap();

    let settings = Settings {
        window: iced::window::Settings {
            size: (1024, 768),
            resizable: true,
            decorations: true,
            min_size: Some((800, 600)),
            max_size: None,
            transparent: false,
            always_on_top: false,
            icon: Some(icon),
        },
        antialiasing: true,
        ..Default::default()
    };

    app::App::run(settings);

    /*
    Console::setup();

    println!("Trying to find the LeagueClient.exe process...");

    let now = std::time::Instant::now();

    while !league_exists(false) {
        std::thread::sleep(std::time::Duration::from_secs(2));
    }

    if now.elapsed().as_millis() < 1000 {
        println!("Found LeagueClient.exe in {}ms!", now.elapsed().as_millis());
    } else {
        println!("Found LeagueClient.exe in {:.2}s!", now.elapsed().as_secs_f64());
    }

    let path = utils::process::get_lock_file_path()?;

    let lock_file_info = utils::lock_file::parse(&path)?;

    let mut lcu = utils::lcu::LCUClient::new(&lock_file_info.token, lock_file_info.port)?;

    let rx = process_worker::spawn();

    let mut key_listener = KeyListener::new();

    #[allow(unused_variables)]
    key_listener
        .register_hotkey(
            Modifiers::CTRL,
            Key::D,
            enclose! {(lcu) move || {
                println!("Lobby Crash Queued...");
                #[cfg(not(debug_assertions))]
                lcu.crash_lobby().unwrap();
                #[cfg(debug_assertions)]
                println!("Debug Assertions are on so you don't go into TFT");

            println!("{}", "Lobby has been dodged, you can ff the TFT game as soon as you load into it.".bright_green());
            }},
        )
        .unwrap();

    key_listener
        .register_hotkey(
            Modifiers::CTRL,
            Key::B,
            enclose! {(lcu) move || {
                println!("ARAM Boost Queued...");
                lcu.send(&Endpoints::AramBoost, &Method::POST, "").unwrap();
                println!("{}", "ARAM Boost Completed...".bright_green());
            }},
        )
        .unwrap();

    key_listener.listen();

    while let Ok(event) = rx.recv() {
        match event {
            Events::Connected => {
                let path = utils::process::get_lock_file_path().unwrap();
                let lock_file_info = utils::lock_file::parse(&path).unwrap();
                lcu.reconnect(&lock_file_info.token, lock_file_info.port);
                println!("{}", "Successfully reconnected to the League Client".bright_green());
            },
            Events::Disconnected => {
                lcu.disconnect();
                println!(
                    "{}",
                    "League Client has been disconnected we will attempt to reconnect to it...".red()
                );
            },
        }
    }*/

    Ok(())
}
