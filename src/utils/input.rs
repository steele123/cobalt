use bindings::Windows::Win32::KeyboardAndMouseInput::GetAsyncKeyState;

pub enum Key {
    D = 0x44,
    B = 0x42,
    CONTROL = 0x11,
}

// key codes https://docs.microsoft.com/en-us/windows/win32/inputdev/virtual-key-codes
// this wrapper will return true if its held down OR pressed
// GetAsyncKeyState is retarded and will return -32767 if key is held, 0 if key
// is not touched and 1 if key is only pressed

pub fn get_key_hold(key: Key) -> bool { unsafe { GetAsyncKeyState(key as i32) == -32767 } }

pub fn get_key_press(key: Key) -> bool { unsafe { GetAsyncKeyState(key as i32) == 1 } }

pub fn get_key_press_or_hold(key: Key) -> bool { unsafe { GetAsyncKeyState(key as i32) != 0 } }
