use std::{collections::HashMap, mem::MaybeUninit};

use bindings::Windows::Win32::{
    KeyboardAndMouseInput::{RegisterHotKey, HOT_KEY_MODIFIERS},
    WindowsAndMessaging::{GetMessageW, HWND, WPARAM},
};

pub enum Key {
    D = 0x44,
    B = 0x42,
    CONTROL = 0x11,
}

pub enum Modifiers {
    ALT = HOT_KEY_MODIFIERS::MOD_ALT.0 as isize,
    CTRL = HOT_KEY_MODIFIERS::MOD_CONTROL.0 as isize,
    SHIFT = HOT_KEY_MODIFIERS::MOD_SHIFT.0 as isize,
}

// key codes https://docs.microsoft.com/en-us/windows/win32/inputdev/virtual-key-codes

pub struct KeyListener {
    last_id: i32,
    handlers: HashMap<i32, Box<dyn Fn()>>,
}

impl KeyListener {
    pub fn new() -> Self {
        Self {
            last_id: 0,
            handlers: HashMap::new(),
        }
    }

    pub fn register_hotkey<Callback: 'static + Fn()>(
        &mut self,
        mods: Modifiers,
        key: Key,
        callback: Callback,
    ) -> eyre::Result<i32> {
        self.last_id += 1;
        let id = self.last_id;
        let result: bool =
            unsafe { RegisterHotKey(HWND::default(), id, HOT_KEY_MODIFIERS::from(mods as u32), key as u32).into() };

        if !result {
            return Err(eyre::eyre!("Failed to register the hotkey".to_string()));
        }

        self.handlers.insert(id, Box::new(callback));

        Ok(id)
    }

    pub fn listen(self) {
        unsafe {
            loop {
                let mut msg = MaybeUninit::assume_init(std::mem::MaybeUninit::uninit());
                while GetMessageW(&mut msg, HWND::default(), 0, 0) != false {
                    if msg.wParam != WPARAM::NULL {
                        if let Some(callback) = self.handlers.get(&(msg.wParam.0 as _)) {
                            callback();
                        }
                    }
                }
            }
        }
    }
}