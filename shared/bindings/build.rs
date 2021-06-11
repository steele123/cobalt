fn main() {
    windows::build!(
        Windows::Win32::UI::WindowsAndMessaging::GetMessageW,
        Windows::Win32::UI::KeyboardAndMouseInput::RegisterHotKey,
        Windows::Win32::Foundation::{CloseHandle, INVALID_HANDLE_VALUE},
        Windows::Win32::System::Threading::{CreateProcessW, OpenProcess, TerminateProcess, PROCESS_ACCESS_RIGHTS, PROCESS_CREATION_FLAGS, STARTUPINFOW_FLAGS},
        Windows::Win32::System::Diagnostics::ToolHelp::{CreateToolhelp32Snapshot, CREATE_TOOLHELP_SNAPSHOT_FLAGS, PROCESSENTRY32, CREATE_TOOLHELP_SNAPSHOT_FLAGS, Process32First, Process32Next, MODULEENTRY32, Module32First},
    );
}
