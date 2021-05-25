fn main() {
    windows::build!(
        Windows::Win32::UI::WindowsAndMessaging::GetMessageW,
        Windows::Win32::UI::KeyboardAndMouseInput::{RegisterHotKey},
        Windows::Win32::System::WindowsProgramming::CloseHandle,
        Windows::Win32::System::Threading::{CreateProcessW, OpenProcess, TerminateProcess},
        Windows::Win32::System::SystemServices::INVALID_HANDLE_VALUE,
        Windows::Win32::System::Diagnostics::ToolHelp::{CreateToolhelp32Snapshot, PROCESSENTRY32, CREATE_TOOLHELP_SNAPSHOT_FLAGS, Process32First, Process32Next, MODULEENTRY32, Module32First},
    );
}
