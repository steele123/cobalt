#![windows_subsystem = "windows"]

use fetcher::Fetcher;
use win_utils::kill_process_by_name;

pub mod fetcher;

pub mod process;

fn main() -> eyre::Result<()> {
    let tool_path: String;

    let cached_version = Fetcher::get_cached_version();

    let latest_version = Fetcher::get_version().unwrap();

    if cached_version == latest_version {
        tool_path = Fetcher::get_tool(true).unwrap();
    } else {
        tool_path = Fetcher::get_tool(false).unwrap();
    }

    Fetcher::save_version(latest_version);

    kill_process_by_name("Cobalt");

    process::create_process(tool_path)?;

    Ok(())
}
