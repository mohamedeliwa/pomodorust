use clap::Parser;
use dialoguer;
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

fn main() {
    let (tx, rx) = mpsc::channel::<Actions>();
    let key_handle = key_handler::run(tx);
    let pomodoro = Args::parse();
    let mut pomodoro = Pomodoro::new(pomodoro.session, pomodoro.pause, rx);

    loop {
        pomodoro.run();
    }

    // let confirmation = dialoguer::Confirm::new()
    //     .with_prompt("Start a session?")
    //     .interact()
    //     .unwrap();
    // if !confirmation {
    //     return;
    // }
    // pomodoro.run();
    // let confirmation = dialoguer::Confirm::new()
    //     .with_prompt("Start a break?")
    //     .interact()
    //     .unwrap();
    // if !confirmation {
    //     return;
    // }
    // pomodoro.run();
    key_handle.join().expect("failed to join key_handler!");
}
