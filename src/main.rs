use clap::Parser;
// use dialoguer;
use std::sync::mpsc;
mod key_handler;
mod pomodoro;
use pomodoro::{Actions, Pomodoro};

#[derive(Parser, Debug)]
struct Args {
    /// How many minutes in a session
    #[arg(short, long, default_value_t = 1)]
    session: u64,
    /// How many minutes in a break
    #[arg(short, long, default_value_t = 1)]
    pause: u64,
}

pub enum MainActions {
    Exit,
}

fn main() {
    // channel to send messages to the pomodoro struct
    let (pomodoro_tx, pomodoro_rx) = mpsc::channel::<Actions>();
    // channel to send message to the main function
    let (main_tx, main_rx) = mpsc::channel::<MainActions>();
    // running the key_kanlder in a separate thread
    let key_handle = key_handler::run(pomodoro_tx, main_tx);
    let pomodoro = Args::parse();
    let mut pomodoro = Pomodoro::new(pomodoro.session, pomodoro.pause, pomodoro_rx);

    loop {
        match main_rx.try_recv() {
            Ok(received) => match received {
                MainActions::Exit => break,
            },
            Err(_) => {}
        }
        pomodoro.run();
    }
    key_handle.join().expect("failed to join key_handler!");
}
