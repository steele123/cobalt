fn main() {
    windows::build!(
        Windows::Win32::WindowsAndMessaging::GetMessageW,
        Windows::Win32::KeyboardAndMouseInput::{RegisterHotKey},
        Windows::Win32::WindowsProgramming::CloseHandle,
        Windows::Win32::SystemServices::{INVALID_HANDLE_VALUE},
        Windows::Win32::ToolHelp::{CreateToolhelp32Snapshot, PROCESSENTRY32, CREATE_TOOLHELP_SNAPSHOT_FLAGS, Process32First, Process32Next, MODULEENTRY32, Module32First},
    );
}
