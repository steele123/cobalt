use std::{ffi::CStr, os::raw::c_ulong};

use bindings::Windows::Win32::System::{
    Diagnostics::{
        ToolHelp,
        ToolHelp::{CreateToolhelp32Snapshot, Process32First, Process32Next, PROCESSENTRY32},
    },
    SystemServices::{CHAR, INVALID_HANDLE_VALUE},
    Threading::{OpenProcess, TerminateProcess, PROCESS_QUERY_INFORMATION, PROCESS_TERMINATE},
    WindowsProgramming::CloseHandle,
};
use eyre::Result;
use std::ffi::CStr;
use std::os::raw::c_ulong;

pub fn get_process_id_by_name(process_name: &str) -> Result<std::os::raw::c_ulong> {
    let mut process_id: std::os::raw::c_ulong = 0;

    let snapshot = unsafe { CreateToolhelp32Snapshot(ToolHelp::TH32CS_SNAPPROCESS, process_id) };

    if snapshot == INVALID_HANDLE_VALUE {
        return Err(std::io::Error::last_os_error().into());
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

pub fn get_processes(processe_names: Vec<&str>) -> eyre::Result<Vec<c_ulong>> {
    let snapshot =
        unsafe { CreateToolhelp32Snapshot(CREATE_TOOLHELP_SNAPSHOT_FLAGS::TH32CS_SNAPPROCESS, 0) };

    if snapshot == INVALID_HANDLE_VALUE {
        return Err(std::io::Error::last_os_error().into());
    }

    let mut entry = PROCESSENTRY32 {
        dwSize: std::mem::size_of::<PROCESSENTRY32>() as u32,
        ..PROCESSENTRY32::default()
    };

    let mut processes: Vec<c_ulong> = Vec::new();

    if unsafe { Process32First(snapshot, &mut entry).into() } {
        loop {
            let current_name = convert_windows_string(entry.szExeFile)?;

            for process in &processe_names {
                if current_name == *process {
                    processes.push(entry.th32ProcessID);
                }
            }

            if !unsafe { Process32Next(snapshot, &mut entry).into() } {
                break;
            }
        }

        unsafe {
            CloseHandle(snapshot);
        }
    }

    Ok(processes)
}

pub fn kill_process_by_name(process_name: &str) {
    let process_id = get_process_id_by_name(process_name).unwrap();

    let process = unsafe { OpenProcess(PROCESS_TERMINATE | PROCESS_QUERY_INFORMATION, false, process_id) };

    unsafe { TerminateProcess(process, 0) };
}

pub fn convert_windows_string<'a, const N: usize>(string: [CHAR; N]) -> Result<&'a str> {
    unsafe { Ok(CStr::from_ptr(string.as_ptr().cast::<i8>()).to_str()?) }
}
