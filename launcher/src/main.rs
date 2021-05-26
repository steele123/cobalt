#![windows_subsystem = "windows"]

use fetcher::Fetcher;

pub mod fetcher;

pub mod process;

fn main() -> eyre::Result<()> {
    Fetcher::launch_tool().unwrap();

    Ok(())
}
