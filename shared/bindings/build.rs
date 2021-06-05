fn main() {
    windows::build!(
        Windows::Win32::UI::WindowsAndMessaging::GetMessageW,
        Windows::Win32::UI::KeyboardAndMouseInput::{RegisterHotKey, MOD_ALT, MOD_SHIFT, MOD_CONTROL},
        Windows::Win32::System::WindowsProgramming::CloseHandle,
        Windows::Win32::System::Threading::{CreateProcessW, OpenProcess, TerminateProcess, PROCESS_QUERY_INFORMATION, PROCESS_TERMINATE, CREATE_NEW_CONSOLE, STARTF_USESTDHANDLES, STARTF_TITLEISAPPID},
        Windows::Win32::System::SystemServices::INVALID_HANDLE_VALUE,
        Windows::Win32::System::Diagnostics::ToolHelp::{CreateToolhelp32Snapshot, PROCESSENTRY32, CREATE_TOOLHELP_SNAPSHOT_FLAGS, Process32First, Process32Next, MODULEENTRY32, Module32First, TH32CS_SNAPPROCESS, TH32CS_SNAPMODULE},
    );
}
