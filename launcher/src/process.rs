use std::io::Error;

use bindings::Windows::Win32::{
    SystemServices::{CreateProcessW, PROCESS_INFORMATION, PWSTR, STARTUPINFOW, STARTUPINFOW_FLAGS},
    WindowsProgramming::{CloseHandle, PROCESS_CREATION_FLAGS},
};

fn str_to_pwstr(string: &str) -> PWSTR {
    PWSTR(std::boxed::Box::<[u16]>::into_raw(
        string
            .encode_utf16()
            .chain(std::iter::once(0))
            .collect::<std::vec::Vec<u16>>()
            .into_boxed_slice(),
    ) as _)
}

pub fn create_process(path: String) -> eyre::Result<()> {
    let flags = PROCESS_CREATION_FLAGS::CREATE_NEW_CONSOLE;

    let mut startup_info = STARTUPINFOW {
        cb: std::mem::size_of::<STARTUPINFOW>() as _,
        dwFlags: STARTUPINFOW_FLAGS::STARTF_USESTDHANDLES | STARTUPINFOW_FLAGS::STARTF_TITLEISAPPID,
        ..Default::default()
    };

    let mut process_info = PROCESS_INFORMATION::default();

    let current_dir = format!("\"--base-path={}\"", std::env::current_dir().unwrap().to_str().unwrap());

    let process_opened: bool = unsafe {
        CreateProcessW(
            PWSTR(std::ptr::null_mut()),
            str_to_pwstr(&format!("\"{}\" {}", path, current_dir)),
            std::ptr::null_mut(),
            std::ptr::null_mut(),
            false,
            flags,
            std::ptr::null_mut(),
            PWSTR(std::ptr::null_mut()),
            &mut startup_info,
            &mut process_info,
        )
        .into()
    };

    if !process_opened {
        return Err(Error::last_os_error().into());
    }

    unsafe {
        CloseHandle(process_info.hProcess);
        CloseHandle(process_info.hThread);
    }

    Ok(())
}
