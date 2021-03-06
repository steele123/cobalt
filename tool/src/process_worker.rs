use std::{
    sync::mpsc::{channel, Receiver, Sender},
    thread,
    time::Duration,
};

use crate::utils::process::league_exists;

const SLEEP_TIME: Duration = Duration::from_millis(25);

pub enum Events {
    Connected,
    Disconnected,
}

pub fn spawn() -> Receiver<Events> {
    let (tx, rx) = channel();

    thread::spawn(move || watch(&tx));

    rx
}
fn sleep() { thread::sleep(SLEEP_TIME); }

fn watch(tx: &Sender<Events>) {
    let mut state = true;
    loop {
        if state != league_exists(true) {
            if state {
                tx.send(Events::Disconnected).unwrap();
            } else {
                tx.send(Events::Connected).unwrap();
            }
            state = league_exists(true);
        }

        sleep();
    }
}
