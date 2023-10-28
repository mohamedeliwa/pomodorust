use std::{
    sync::mpsc::Sender,
    thread::{self, JoinHandle},
};

use console::{Key, Term};

use crate::{pomodoro::Actions, MainActions};

pub fn run(pomodoro_tx: Sender<Actions>, main_tx: Sender<MainActions>) -> JoinHandle<()> {
    thread::spawn(move || {
        let mut paused: bool = false;
        loop {
            let key = Term::stdout().read_key().unwrap();
            match key {
                // capturing clicking space key, for pausing and resuming
                Key::Char(' ') => {
                    if paused {
                        pomodoro_tx
                            .send(Actions::Resume)
                            .expect("failed to send pause action msg!");
                        paused = false;
                    } else {
                        pomodoro_tx
                            .send(Actions::Puase)
                            .expect("failed to send pause action msg!");
                        paused = true;
                    }
                }
                // capturing q char for quitting the program
                Key::Char('q') => {
                    pomodoro_tx
                        .send(Actions::Exit)
                        .expect("failed to send exit pomodoro action msg!");
                    main_tx
                        .send(MainActions::Exit)
                        .expect("failed to send exit action msg!");
                    return;
                }
                // skipping any other key
                _ => {}
            }
        }
    })
}
