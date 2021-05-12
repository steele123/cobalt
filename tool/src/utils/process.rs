use std::{ffi::CStr, io::Error};

use bindings::Windows::Win32::{
    SystemServices::{CHAR, INVALID_HANDLE_VALUE},
    ToolHelp::{
        CreateToolhelp32Snapshot, Module32First, Process32First, Process32Next, CREATE_TOOLHELP_SNAPSHOT_FLAGS,
        MODULEENTRY32, PROCESSENTRY32,
    },
    WindowsProgramming::CloseHandle,
};
use eyre::Result;
use win_utils::{convert_windows_string, get_process_id_by_name};

pub fn get_lock_file_path() -> Result<String> {
    let process_id = get_process_id_by_name("LeagueClient.exe")?;

    let module_snapshot = unsafe { CreateToolhelp32Snapshot(CREATE_TOOLHELP_SNAPSHOT_FLAGS::TH32CS_SNAPMODULE, process_id) };

    let mut module_entry = MODULEENTRY32 {
        dwSize: std::mem::size_of::<MODULEENTRY32>() as u32,
        ..MODULEENTRY32::default()
    };

    if unsafe { Module32First(module_snapshot, &mut module_entry).into() } {
        Ok(convert_windows_string(module_entry.szExePath)?.into())
    } else {
        Err(Error::last_os_error().into())
    }
}

pub fn league_exists(ui_process: bool) -> bool {
    if ui_process {
        get_process_id_by_name("LeagueClientUx.exe").unwrap() != 0
    } else {
        get_process_id_by_name("LeagueClient.exe").unwrap() != 0
    }
}
