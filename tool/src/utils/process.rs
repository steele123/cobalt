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

pub fn convert_windows_string<'a, const N: usize>(string: [CHAR; N]) -> Result<&'a str> {
    unsafe { Ok(CStr::from_ptr(string.as_ptr().cast::<i8>()).to_str()?) }
}

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

pub fn get_process_id_by_name(process_name: &str) -> Result<std::os::raw::c_ulong> {
    let mut process_id: std::os::raw::c_ulong = 0;

    let snapshot = unsafe { CreateToolhelp32Snapshot(CREATE_TOOLHELP_SNAPSHOT_FLAGS::TH32CS_SNAPPROCESS, process_id) };

    if snapshot == INVALID_HANDLE_VALUE {
        return Err(Error::last_os_error().into());
    }

    let mut entry = PROCESSENTRY32 {
        dwSize: std::mem::size_of::<PROCESSENTRY32>() as u32,
        ..PROCESSENTRY32::default()
    };

    if unsafe { Process32First(snapshot, &mut entry).into() } {
        process_id = loop {
            let current_name = convert_windows_string(entry.szExeFile)?;
            if current_name == process_name {
                break entry.th32ProcessID;
            }

            if !unsafe { Process32Next(snapshot, &mut entry).into() } {
                break 0;
            }
        };

        unsafe {
            CloseHandle(snapshot);
        }
    }

    Ok(process_id)
}
