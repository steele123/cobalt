#![windows_subsystem = "windows"]

use std::process::Command;

use crate::fetcher::Fetcher;

pub mod fetcher;

fn main() -> eyre::Result<()> {
    let version = Fetcher::get_version()?;

    let tool_path = Fetcher::download_tool()?;

    Command::new(tool_path).arg("--launcher").spawn()?;

    Ok(())
}
