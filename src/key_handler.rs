use std::{
    sync::mpsc::Sender,
    thread::{self, JoinHandle},
};

use console::{Key, Term};

use crate::pomodoro::Actions;

pub fn run(tx: Sender<Actions>) -> JoinHandle<()> {
    thread::spawn(move || {
        let mut paused: bool = false;
        loop {
            let key = Term::stdout().read_key().unwrap();
            match key {
                // capturing clicking space key, for pausing and resuming
                Key::Char(' ') => {
                    if paused {
                        tx.send(Actions::Resume)
                            .expect("failed to send pause action msg!");
                        paused = false;
                    } else {
                        tx.send(Actions::Puase)
                            .expect("failed to send pause action msg!");
                        paused = true;
                    }
                }
                // capturing q char for quitting the program
                // Key::Char('q') => return,
                // skipping any other key
                _ => {}
            }
        }
    })
}
