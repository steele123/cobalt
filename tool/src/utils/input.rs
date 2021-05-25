use std::{collections::HashMap, mem::MaybeUninit};

use bindings::Windows::Win32::UI::{
    KeyboardAndMouseInput::{RegisterHotKey, HOT_KEY_MODIFIERS, MOD_ALT, MOD_CONTROL, MOD_SHIFT},
    WindowsAndMessaging::{GetMessageW, HWND, WPARAM},
};

#[derive(Copy, Clone)]
pub enum Key {
    D = 0x44,
    B = 0x42,
    CONTROL = 0x11,
}

#[derive(Copy, Clone)]
pub enum Modifiers {
    ALT = MOD_ALT.0 as isize,
    CTRL = MOD_CONTROL.0 as isize,
    SHIFT = MOD_SHIFT.0 as isize,
}

// key codes https://docs.microsoft.com/en-us/windows/win32/inputdev/virtual-key-codes

pub struct KeyListener {
    last_id: i32,
    handlers: HashMap<i32, HotKey>,
}

struct HotKey {
    id: i32,
    mods: Modifiers,
    key: Key,
    callback: Box<dyn Fn() + Send>,
}

impl KeyListener {
    pub fn new() -> Self {
        Self {
            last_id: 0,
            handlers: HashMap::new(),
        }
    }

    pub fn register_hotkey<F: 'static>(&mut self, mods: Modifiers, key: Key, callback: F) -> eyre::Result<i32>
    where
        F: Fn() + Send,
    {
        self.last_id += 1;

        let hk = HotKey {
            id: self.last_id,
            mods,
            key,
            callback: Box::new(callback),
        };

        self.handlers.insert(self.last_id, hk);

        Ok(self.last_id)
    }

    pub fn listen(self) {
        std::thread::spawn(move || unsafe {
            for (i, hk) in &self.handlers {
                RegisterHotKey(HWND::default(), *i, HOT_KEY_MODIFIERS::from(hk.mods as u32), hk.key as u32);
            }

            loop {
                let mut msg = MaybeUninit::assume_init(std::mem::MaybeUninit::uninit());
                while GetMessageW(&mut msg, HWND::default(), 0, 0) != false {
                    if msg.wParam != WPARAM::NULL {
                        if let Some(hotkey) = self.handlers.get(&(msg.wParam.0 as _)) {
                            (hotkey.callback)();
                        }
                    }
                }
            }
        });
    }
}
