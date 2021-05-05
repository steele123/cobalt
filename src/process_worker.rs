use std::{thread, time::Duration};

use crate::utils::process::league_exists;

pub static WORKER: ProcessWorker = ProcessWorker { connected: true };

const SLEEP_TIME_MS: u64 = 1000;

pub struct ProcessWorker {
    pub connected: bool,
}

impl ProcessWorker {
    fn watch(&mut self) {
        loop {
            let league_exists = league_exists();

            if league_exists && self.connected {
                self.sleep();
                continue;
            }

            if league_exists {
                self.connected = true;
            } else if self.connected {
                self.connected = false;
            }

            self.sleep();
        }
    }

    pub fn spawn(&'static mut self) { thread::spawn(move || self.watch()); }

    fn sleep(&self) { thread::sleep(Duration::from_millis(SLEEP_TIME_MS)); }
}
