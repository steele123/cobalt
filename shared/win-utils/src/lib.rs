use std::ffi::CStr;

use bindings::Windows::Win32::{
    Foundation::{CloseHandle, INVALID_HANDLE_VALUE},
    System::{
        Diagnostics::{
            ToolHelp,
            ToolHelp::{CreateToolhelp32Snapshot, Process32First, Process32Next, PROCESSENTRY32},
        },
        SystemServices::CHAR,
        Threading::{OpenProcess, TerminateProcess, PROCESS_QUERY_INFORMATION, PROCESS_TERMINATE},
    },
};
use eyre::Result;

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

pub fn kill_process_by_name(process_name: &str) {
    let process_id = get_process_id_by_name(process_name).unwrap();

    let process = unsafe { OpenProcess(PROCESS_TERMINATE | PROCESS_QUERY_INFORMATION, false, process_id) };

    unsafe { TerminateProcess(process, 0) };
}

pub fn convert_windows_string<'a, const N: usize>(string: [CHAR; N]) -> Result<&'a str> {
    unsafe { Ok(CStr::from_ptr(string.as_ptr().cast::<i8>()).to_str()?) }
}
